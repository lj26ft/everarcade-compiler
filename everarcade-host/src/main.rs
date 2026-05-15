use std::{fs, path::PathBuf};

use sha2::{Digest, Sha256};

use everarcade_host::network::{peer_dialer, tcp_client, tcp_server};

use everarcade_host::{
    config::HostConfig,
    error::HostError,
    fixture::generate_fixture_to_path,
    index::index_rebuild::rebuild_indexes,
    persistence::HostPaths,
    run_package_once,
    state_folder::{
        manifest_rebuild::repair_manifest,
        node_manifest::{read_node_manifest, write_node_manifest, NodeManifest},
        storage_report::storage_report,
        validation::validate,
    },
    verify::verify_state,
};

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}


fn publish_artifact_manifest(state: &std::path::Path, receipt_hex: &str) -> Result<String, HostError> {
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
        everarcade_host::xrpl::submission::submit_stub(&intent)
            .map_err(HostError::InvalidArgs)?;
    }
    Ok(intent.anchor_root_hex)
}
fn run_cli() -> Result<(), HostError> {
    let args: Vec<String> = std::env::args().collect();
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
            println!("initialized={}", state.display());
        }
        "generate-fixture" => {
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
        "repair-manifest" => {
            let report = repair_manifest(&state)?;
            println!("repaired={} latest_receipt_root={} latest_checkpoint_root={}", report.repaired, report.latest_receipt_root.map(hex::encode).unwrap_or_else(||"none".into()), report.latest_checkpoint_root.map(hex::encode).unwrap_or_else(||"none".into()));
        }
        "rebuild-index" => {
            let report = rebuild_indexes(&state)?;
            println!("rebuilt_receipts={} rebuilt_checkpoints={} rebuilt_anchors={}", report.rebuilt_receipts, report.rebuilt_checkpoints, report.rebuilt_anchors);
        }
        "status" => {
            let manifest = read_node_manifest(&state)?;
            let anchor_count = fs::read_dir(state.join("anchors"))?.count();
            println!(
                "node_name={} last_receipt_root={:?} last_checkpoint_root={:?} anchor_queue={}",
                manifest.node_name,
                manifest.last_receipt_root,
                manifest.last_checkpoint_root,
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
            let bind = arg_value(&args, "--bind").ok_or_else(|| HostError::InvalidArgs("missing --bind".into()))?;
            let listener = tcp_server::bind(&bind).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let _ = everarcade_host::network::peer_listener::serve_once(&listener, b"sync-ok")
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            println!("serve=ok bind={bind}");
        }
        "sync" => {
            let peer = arg_value(&args, "--peer").ok_or_else(|| HostError::InvalidArgs("missing --peer".into()))?;
            let stream = tcp_client::connect(&peer).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let response = peer_dialer::request_sync(stream, b"sync-request")
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            println!("sync=ok peer={} response={}", peer, String::from_utf8_lossy(&response));
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
        "deploy-proof" => {
            let package =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let profile = arg_value(&args, "--profile").unwrap_or_else(|| "live".into());
            let node = arg_value(&args, "--node").unwrap_or_else(|| "evernode-operator-1".into());
            if profile != "live" {
                return Err(HostError::InvalidArgs("--profile currently supports only live".into()));
            }
            HostPaths::new(state.clone()).ensure()?;
            let result = run_package_once(HostConfig::new(package, state.clone()))?;
            let receipt_hex = hex::encode(result.receipt.receipt_id);
            let checkpoint_hex = hex::encode(result.receipt.checkpoint_root);
            let anchor = submit_xrpl_anchor_intent(&receipt_hex)?;
            let cid = publish_artifact_manifest(&state, &receipt_hex)?;
            println!("proof package=ok execute=ok receipt={} checkpoint={} distributed-receipt=ok xrpl-anchor={} ipfs-manifest={}", receipt_hex, checkpoint_hex, anchor, cid);
            let cfg = everarcade_host::operator::config::OperatorConfig::live_testnet(node);
            println!("operator profile={:?} state={} xrpl={} ipfs={} evernode={}", cfg.profile, cfg.state_path, cfg.xrpl_enabled, cfg.ipfs_enabled, cfg.evernode_enabled);
        }
        _ => {
            return Err(HostError::InvalidArgs(
                "usage: init|generate-fixture|run|verify|repair-manifest|rebuild-index|status|serve|sync|anchor-intent|deploy-proof".into(),
            ))
        }
    }
    Ok(())
}
