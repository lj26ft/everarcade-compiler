use std::{fs, path::Path, path::PathBuf};

use sha2::{Digest, Sha256};

use everarcade_host::network::{peer_dialer, tcp_client, tcp_server};

use everarcade_host::{
    config::HostConfig,
    error::HostError,
    fixture::generate_fixture_to_path,
    index::index_rebuild::rebuild_indexes,
    persistence::HostPaths,
    run_package_once,
    replay_engine::verify_receipt_replay,
    state_folder::{
        manifest_rebuild::repair_manifest,
        node_manifest::{read_node_manifest, write_node_manifest, NodeManifest},
        storage_report::storage_report,
        validation::validate,
    },
    verify::verify_state,
};

const HELP_TEXT: &str = r#"EverArcade Host Operator

Commands:
  init --state <path>
  generate-fixture --output <path>
  run --package <path> --state <path>
  verify --state <path>
  status --state <path>
  deploy-proof --package <path> --state <path>
  debug --state <path>
  replay-verify --package <path> --receipt <path>
  restore-verify --package <path> --receipt <path> --checkpoint <path>
  doctor --state <path>

Examples:
  everarcade-host init --state ~/.everarcade
  everarcade-host generate-fixture --output /tmp/everarcade-package.bin
  everarcade-host run --package /tmp/everarcade-package.bin --state ~/.everarcade
  everarcade-host verify --state ~/.everarcade"#;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn publish_artifact_manifest(
    state: &std::path::Path,
    receipt_hex: &str,
) -> Result<String, HostError> {
    let manifest = serde_json::json!({
        "receipt": receipt_hex,
        "deterministic": true,
        "state": state.display().to_string(),
    });
    let bytes = serde_json::to_vec(&manifest).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
    let cid = everarcade_host::ipfs::ipfs_publish::publish_bytes(&bytes)
        .unwrap_or_else(|| format!("dryrun:{}", hex::encode(Sha256::digest(&bytes))));
    Ok(cid)
}

fn submit_xrpl_anchor_intent(receipt_hex: &str) -> Result<String, HostError> {
    let payload_hex = hex::encode(format!("everarcade-anchor:{receipt_hex}"));
    let intent = everarcade_host::xrpl::anchor_intent::XrplAnchorIntent {
        receipt_id_hex: receipt_hex.to_string(),
        anchor_root_hex: receipt_hex.to_string(),
        payload_hex,
    };
    if everarcade_host::xrpl::xrpl_live::live_enabled() {
        everarcade_host::xrpl::submission::submit_stub(&intent).map_err(HostError::InvalidArgs)?;
    }
    Ok(intent.anchor_root_hex)
}

fn latest_root_hex(state: &Path, folder: &str) -> String {
    let path = state.join(folder);
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
            .filter(|name| name.ends_with(".json"))
            .map(|name| name.trim_end_matches(".json").to_string())
            .max()
            .unwrap_or_else(|| "none".into()),
        Err(_) => "none".into(),
    }
}

fn run_cli() -> Result<(), HostError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{HELP_TEXT}");
        return Ok(());
    }
    let cmd = args
        .get(1)
        .ok_or_else(|| HostError::InvalidArgs("missing command".into()))?;
    let state = PathBuf::from(arg_value(&args, "--state").unwrap_or_else(|| ".everarcade".into()));
    match cmd.as_str() {
        "init" => {
            HostPaths::new(state.clone()).ensure()?;
            if !validate(&state) {
                return Err(HostError::InvalidStateFolder);
            }
            write_node_manifest(&state, &NodeManifest::new("everarcade-node"))?;
            println!("init=ok state={}", state.display());
        }
        "generate-fixture" => {
            /* unchanged */
            let output = PathBuf::from(
                arg_value(&args, "--output")
                    .ok_or_else(|| HostError::InvalidArgs("missing --output".into()))?,
            );
            let result = generate_fixture_to_path(&output)?;
            println!(
                "fixture=ok output={} package_root={} replay_root={} checkpoint_root={}",
                result.output_path,
                hex::encode(result.package_root),
                hex::encode(result.replay_root),
                hex::encode(result.checkpoint_root)
            );
        }
        "run" => {
            let package =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            if !package.exists() {
                return Err(HostError::MissingPackage);
            }
            let result = run_package_once(HostConfig::new(package, state.clone()))?;
            let mut manifest =
                read_node_manifest(&state).unwrap_or_else(|_| NodeManifest::new("everarcade-node"));
            manifest.last_receipt_root = Some(hex::encode(result.receipt.receipt_id));
            manifest.last_checkpoint_root = Some(hex::encode(result.receipt.checkpoint_root));
            manifest.last_anchor_root = Some(hex::encode(result.receipt.receipt_id));
            write_node_manifest(&state, &manifest)?;
            println!("receipt={}", hex::encode(result.receipt.receipt_id));
        }
        "verify" => {
            let report = verify_state(&state)?;
            if !report.all_valid() {
                return Err(HostError::VerificationFailed(format!(
                    "package_valid={} receipt_valid={} checkpoint_valid={} anchor_valid={}",
                    report.package_valid,
                    report.receipt_valid,
                    report.checkpoint_valid,
                    report.anchor_valid
                )));
            }
            let manifest =
                read_node_manifest(&state).unwrap_or_else(|_| NodeManifest::new("everarcade-node"));
            write_node_manifest(&state, &manifest)?;
            println!("verify=ok");
        }
        "debug" => {
            let manifest =
                read_node_manifest(&state).unwrap_or_else(|_| NodeManifest::new("everarcade-node"));
            let anchor_count = fs::read_dir(state.join("anchors"))
                .map(|x| x.count())
                .unwrap_or(0);
            let distributed = fs::read_dir(state.join("distributed-receipts"))
                .map(|x| x.count())
                .unwrap_or(0);
            println!("version={}", env!("CARGO_PKG_VERSION"));
            println!("state_path={}", state.display());
            println!("state_exists={}", state.exists());
            println!(
                "node_manifest_exists={}",
                state.join("node_manifest.json").exists()
            );
            println!(
                "latest_receipt_root={}",
                manifest
                    .last_receipt_root
                    .unwrap_or_else(|| latest_root_hex(&state, "receipts"))
            );
            println!(
                "latest_checkpoint_root={}",
                manifest
                    .last_checkpoint_root
                    .unwrap_or_else(|| latest_root_hex(&state, "checkpoints"))
            );
            println!("anchor_queue_count={anchor_count}");
            println!("distributed_receipt_count={distributed}");
        }
        "doctor" => {
            let mut failures = Vec::new();
            if !validate(&state) {
                failures.push("state layout invalid");
            }
            if read_node_manifest(&state).is_err() {
                failures.push("manifest unreadable");
            }
            if latest_root_hex(&state, "receipts") == "none" {
                failures.push("latest receipt missing");
            }
            if latest_root_hex(&state, "checkpoints") == "none" {
                failures.push("latest checkpoint missing");
            }
            if verify_state(&state).map(|r| r.all_valid()).unwrap_or(false) != true {
                failures.push("verify failed");
            }
            let anchor_count = fs::read_dir(state.join("anchors"))
                .map(|x| x.count())
                .unwrap_or(0);
            if anchor_count > 0 {
                let has_anchor_json = fs::read_dir(state.join("anchors"))
                    .map(|x| {
                        x.filter_map(Result::ok)
                            .any(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
                    })
                    .unwrap_or(false);
                if !has_anchor_json {
                    failures.push("anchor intent missing while anchor queue > 0");
                }
            }
            if repair_manifest(&state).is_err() || rebuild_indexes(&state).is_err() {
                failures.push("derived indexes/manifests not rebuildable");
            }
            if failures.is_empty() {
                println!("doctor=ok");
            } else {
                return Err(HostError::InvalidArgs(format!(
                    "doctor=fail [{}]",
                    failures.join(", ")
                )));
            }
        }
        "repair-manifest" => {
            let report = repair_manifest(&state)?;
            println!(
                "repaired={} latest_receipt_root={} latest_checkpoint_root={}",
                report.repaired,
                report
                    .latest_receipt_root
                    .map(hex::encode)
                    .unwrap_or_else(|| "none".into()),
                report
                    .latest_checkpoint_root
                    .map(hex::encode)
                    .unwrap_or_else(|| "none".into())
            );
        }
        "rebuild-index" => {
            let report = rebuild_indexes(&state)?;
            println!(
                "rebuilt_receipts={} rebuilt_checkpoints={} rebuilt_anchors={}",
                report.rebuilt_receipts, report.rebuilt_checkpoints, report.rebuilt_anchors
            );
        }
        "status" => {
            let manifest = read_node_manifest(&state)?;
            let anchor_count = fs::read_dir(state.join("anchors"))?.count();
            println!(
                "status node_name={} last_receipt_root={} last_checkpoint_root={} anchor_queue={}",
                manifest.node_name,
                manifest
                    .last_receipt_root
                    .clone()
                    .unwrap_or_else(|| "none".into()),
                manifest
                    .last_checkpoint_root
                    .clone()
                    .unwrap_or_else(|| "none".into()),
                anchor_count
            );
            if args.iter().any(|arg| arg == "--storage") {
                let report = storage_report(&state)?;
                println!(
                    "storage receipts={} checkpoints={} anchors={} total_bytes={}",
                    report.receipt_count,
                    report.checkpoint_count,
                    report.anchor_count,
                    report.total_bytes
                );
            }
        }
        "serve" => {
            let bind = arg_value(&args, "--bind")
                .ok_or_else(|| HostError::InvalidArgs("missing --bind".into()))?;
            let listener =
                tcp_server::bind(&bind).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let _ = everarcade_host::network::peer_listener::serve_once(&listener, b"sync-ok")
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            println!("serve=ok bind={bind}");
        }
        "sync" => {
            let peer = arg_value(&args, "--peer")
                .ok_or_else(|| HostError::InvalidArgs("missing --peer".into()))?;
            let stream =
                tcp_client::connect(&peer).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let response = peer_dialer::request_sync(stream, b"sync-request")
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            println!(
                "sync=ok peer={} response={}",
                peer,
                String::from_utf8_lossy(&response)
            );
        }
        "anchor-intent" => {
            let manifest = read_node_manifest(&state)?;
            let rid = manifest
                .last_anchor_root
                .ok_or(HostError::AnchorIntentMissing)?;
            let p = state.join("anchors").join(format!("{rid}.json"));
            if !p.exists() {
                return Err(HostError::AnchorIntentMissing);
            }
            println!("{}", p.display());
        }
        "restore-verify" => {
            let package = PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let receipt_path = PathBuf::from(arg_value(&args, "--receipt").ok_or_else(|| HostError::InvalidArgs("missing --receipt".into()))?);
            let checkpoint_path = PathBuf::from(arg_value(&args, "--checkpoint").ok_or_else(|| HostError::InvalidArgs("missing --checkpoint".into()))?);
            match execution_core::persistence::restore_and_replay(&package, &receipt_path, &checkpoint_path) {
                Ok(report) if report.state_match => {
                    println!("restore_verify=ok");
                    println!("checkpoint_match={}", report.checkpoint_match);
                    println!("receipt_match={}", report.receipt_match);
                    println!("state_match={}", report.state_match);
                }
                Ok(report) => {
                    println!("restore_verify=failed");
                    let field = if !report.checkpoint_match { "checkpoint_root" } else if !report.receipt_match { "receipt" } else { "state" };
                    println!("field={field}");
                    println!("expected=true");
                    println!("actual=false");
                    return Err(HostError::VerificationFailed(field.into()));
                }
                Err(e) => {
                    println!("restore_verify=failed");
                    println!("field=checkpoint_root");
                    println!("expected=match");
                    println!("actual={e}");
                    return Err(HostError::VerificationFailed(e.to_string()));
                }
            }
        }
        "replay-verify" => {
            let package =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let receipt_path = PathBuf::from(
                arg_value(&args, "--receipt")
                    .ok_or_else(|| HostError::InvalidArgs("missing --receipt".into()))?,
            );
            let report = verify_receipt_replay(&package, &receipt_path)?;
            if !report.verified() {
                return Err(HostError::VerificationFailed(format!(
                    "receipt_canonical_valid={} package_matches_receipt={} deterministic_replay_match={}",
                    report.receipt_canonical_valid,
                    report.package_matches_receipt,
                    report.deterministic_replay_match
                )));
            }
            println!("replay_verify=ok");
        }
        "deploy-proof" => {
            let package =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let profile = arg_value(&args, "--profile").unwrap_or_else(|| "dry-run".into());
            let node = arg_value(&args, "--node").unwrap_or_else(|| "evernode-operator-1".into());
            HostPaths::new(state.clone()).ensure()?;
            let result = run_package_once(HostConfig::new(package, state.clone()))?;
            let receipt_hex = hex::encode(result.receipt.receipt_id);
            let checkpoint_hex = hex::encode(result.receipt.checkpoint_root);
            let anchor = submit_xrpl_anchor_intent(&receipt_hex)?;
            let cid = publish_artifact_manifest(&state, &receipt_hex)?;
            let deployment_manifest = serde_json::json!({"manifest_version": 1,"runtime": "everarcade-host","profile": profile,"node": node,"receipt": receipt_hex,"checkpoint": checkpoint_hex,"xrpl_anchor": anchor,"ipfs_manifest": cid,});
            let manifest_path = state.join("deployment-manifest.json");
            fs::write(
                &manifest_path,
                serde_json::to_vec_pretty(&deployment_manifest)
                    .map_err(|e| HostError::InvalidArgs(e.to_string()))?,
            )?;
            println!("deploy_proof=ok manifest={} receipt={} checkpoint={} xrpl_anchor={} ipfs_manifest={}", manifest_path.display(), deployment_manifest["receipt"], deployment_manifest["checkpoint"], deployment_manifest["xrpl_anchor"], deployment_manifest["ipfs_manifest"]);
            let cfg = everarcade_host::operator::config::OperatorConfig::live_testnet(node);
            println!(
                "operator profile={:?} state={} xrpl={} ipfs={} evernode={}",
                cfg.profile,
                cfg.state_path,
                cfg.xrpl_enabled,
                cfg.ipfs_enabled,
                cfg.evernode_enabled
            );
        }
        _ => {
            return Err(HostError::InvalidArgs(
                "unknown command (run --help)".into(),
            ))
        }
    }
    Ok(())
}
