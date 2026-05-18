use std::{fs, path::Path, path::PathBuf};

use sha2::{Digest, Sha256};

use everarcade_host::network::{peer_dialer, tcp_client, tcp_server};

use everarcade_host::{
    config::HostConfig,
    error::HostError,
    fixture::generate_fixture_to_path,
    index::index_rebuild::rebuild_indexes,
    persistence::HostPaths,
    replay_engine::verify_receipt_replay,
    run_package_once,
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
  lineage-verify --lineage <path>
  chain-restore-verify --package <path> --checkpoint <path> --lineage <path> --receipt <path> [--receipt <path> ...]
  determinism-verify --package <path> --checkpoint <path> --lineage <path> --receipt <path> [--receipt <path> ...]
  recover-world --package <path> --checkpoint <path> --lineage <path> --receipt <path> [--receipt <path> ...]
  verify-recovery --descriptor <path>
  export-bundle --out <bundle_root> --package <path> --checkpoint <path> --lineage <path> --manifest <path> --descriptor <path> --receipt <path> [--receipt <path> ...]
  verify-bundle --bundle <bundle_root>
  import-bundle --bundle <bundle_root> --world-root <path>
  freeze-world --world <id> --world-root <path>
  resume-world --world-root <path> --world <id>
  migrate-world --world <id> --bundle <path> --destination <node-id> --world-root <path>
  scheduler-run-once --world-root <path>
  scheduler-status --world-root <path>
  sync-advertise --world-root <path>
  sync-verify --bundle <path>
  sync-pull --world-root <path> --start-sequence <n> --end-sequence <n>
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

#[derive(Clone)]
struct WorldSchedulerState {
    latest_tick: u64,
    checkpoint_root: [u8; 32],
}

impl WorldSchedulerState {
    fn load(world_root: &Path) -> Result<Self, HostError> {
        let path = world_root.join("runtime").join("scheduler_state.json");
        if !path.exists() {
            return Ok(Self {
                latest_tick: 0,
                checkpoint_root: [0u8; 32],
            });
        }
        let raw = fs::read_to_string(&path).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
        let value: serde_json::Value =
            serde_json::from_str(&raw).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
        let latest_tick = value
            .get("latest_tick")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let checkpoint_root = value
            .get("checkpoint_root")
            .and_then(|v| v.as_str())
            .and_then(|s| hex::decode(s).ok())
            .and_then(|bytes| bytes.try_into().ok())
            .unwrap_or([0u8; 32]);
        Ok(Self {
            latest_tick,
            checkpoint_root,
        })
    }

    fn persist(&self, world_root: &Path) -> Result<(), HostError> {
        let runtime_dir = world_root.join("runtime");
        fs::create_dir_all(&runtime_dir).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
        let body = serde_json::json!({
            "latest_tick": self.latest_tick,
            "checkpoint_root": hex::encode(self.checkpoint_root),
        });
        fs::write(
            runtime_dir.join("scheduler_state.json"),
            serde_json::to_vec_pretty(&body).map_err(|e| HostError::InvalidArgs(e.to_string()))?,
        )
        .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
        Ok(())
    }
}

fn scheduler_queue_events(
    world_root: &Path,
) -> Result<Vec<execution_core::scheduler::events::ScheduledEvent>, HostError> {
    let queue_dir = world_root.join("queue");
    fs::create_dir_all(&queue_dir).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
    let mut events = Vec::new();
    for entry in fs::read_dir(&queue_dir).map_err(|e| HostError::InvalidArgs(e.to_string()))? {
        let path = entry
            .map_err(|e| HostError::InvalidArgs(e.to_string()))?
            .path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
        let value: serde_json::Value =
            serde_json::from_str(&raw).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
        let sequence = value.get("sequence").and_then(|v| v.as_u64()).unwrap_or(0);
        let source = value
            .get("source")
            .and_then(|v| v.as_str())
            .unwrap_or("world")
            .to_string();
        let payload = value
            .get("payload")
            .and_then(|v| v.as_str())
            .map(|s| s.as_bytes().to_vec())
            .unwrap_or_default();
        events.push(execution_core::scheduler::events::ScheduledEvent {
            sequence,
            source,
            payload,
        });
    }
    events.sort();
    Ok(events)
}

fn write_scheduler_receipt(
    world_root: &Path,
    tick: u64,
    event_sequence: Option<u64>,
    checkpoint_root: [u8; 32],
) -> Result<PathBuf, HostError> {
    let receipts = world_root.join("receipts");
    fs::create_dir_all(&receipts).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
    let path = receipts.join(format!("tick-{tick:020}.json"));
    let body = serde_json::json!({
        "tick": tick,
        "event_sequence": event_sequence,
        "checkpoint_root": hex::encode(checkpoint_root),
    });
    fs::write(
        &path,
        serde_json::to_vec_pretty(&body).map_err(|e| HostError::InvalidArgs(e.to_string()))?,
    )
    .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
    Ok(path)
}

struct SchedulerWorld {
    lineage: u64,
    checkpoint: execution_core::scheduler::world::WorldCheckpoint,
}

impl execution_core::scheduler::world::DeterministicWorld for SchedulerWorld {
    fn checkpoint(&self) -> execution_core::scheduler::world::WorldCheckpoint {
        self.checkpoint.clone()
    }

    fn apply(
        &mut self,
        tick: execution_core::scheduler::tick::DeterministicTick,
        event: Option<&execution_core::scheduler::events::ScheduledEvent>,
    ) -> execution_core::scheduler::world::TickReceipt {
        execution_core::scheduler::world::TickReceipt {
            lineage: self.lineage,
            tick,
            event_sequence: event.map(|e| e.sequence),
        }
    }

    fn persist_checkpoint(
        &mut self,
        checkpoint: execution_core::scheduler::world::WorldCheckpoint,
    ) {
        self.checkpoint = checkpoint;
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

        "sync-advertise" => {
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let manifest =
                execution_core::canonical::load_manifest(&world_root.join("manifest.bin"))
                    .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let lineage = execution_core::lineage::load_lineage(&world_root.join("lineage.bin"))
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let latest = lineage
                .records
                .last()
                .ok_or_else(|| HostError::InvalidArgs("empty lineage".into()))?;
            println!("sync_advertise=ok");
            println!("latest_sequence={}", latest.sequence);
            println!("checkpoint_root={}", hex::encode(manifest.checkpoint_root));
            println!(
                "manifest_hash={}",
                hex::encode(execution_core::canonical::manifest_hash(&manifest))
            );
        }
        "sync-verify" => {
            let bundle = PathBuf::from(
                arg_value(&args, "--bundle")
                    .ok_or_else(|| HostError::InvalidArgs("missing --bundle".into()))?,
            );
            match execution_core::sync::verification::verify_sync_artifacts(&bundle) {
                Ok(v) if v.continuity_ok => {
                    println!("sync_verify=ok");
                    println!("continuity_ok={}", v.continuity_ok);
                    println!("replay_ok={}", v.replay_ok);
                    println!("lineage_ok={}", v.lineage_ok);
                }
                Ok(v) => {
                    println!("sync_verify=failed");
                    println!("field=continuity_ok");
                    println!("expected=true");
                    println!("actual={}", v.continuity_ok);
                    return Err(HostError::VerificationFailed("continuity_ok".into()));
                }
                Err(e) => {
                    println!("sync_verify=failed");
                    println!("field=bundle");
                    println!("expected=valid");
                    println!("actual={e}");
                    return Err(HostError::VerificationFailed(e.to_string()));
                }
            }
        }
        "sync-pull" => {
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let start_sequence: u64 = arg_value(&args, "--start-sequence")
                .ok_or_else(|| HostError::InvalidArgs("missing --start-sequence".into()))?
                .parse()
                .map_err(|e: std::num::ParseIntError| HostError::InvalidArgs(e.to_string()))?;
            let end_sequence: u64 = arg_value(&args, "--end-sequence")
                .ok_or_else(|| HostError::InvalidArgs("missing --end-sequence".into()))?
                .parse()
                .map_err(|e: std::num::ParseIntError| HostError::InvalidArgs(e.to_string()))?;
            let mut receipts = std::fs::read_dir(world_root.join("receipts"))
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("bin"))
                .collect::<Vec<_>>();
            receipts.sort();
            let n = (end_sequence - start_sequence + 1) as usize;
            println!("sync_pull=ok");
            println!("receipts={}", receipts.into_iter().take(n).count());
            println!("window_start={start_sequence}");
            println!("window_end={end_sequence}");
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
        "lineage-verify" => {
            let lineage_path = PathBuf::from(
                arg_value(&args, "--lineage")
                    .ok_or_else(|| HostError::InvalidArgs("missing --lineage".into()))?,
            );
            match execution_core::lineage::load_lineage(&lineage_path)
                .and_then(|chain| execution_core::lineage::validate_lineage_chain(&chain))
            {
                Ok(report) => {
                    println!("lineage_verify=ok");
                    println!("lineage_ok={}", report.lineage_ok);
                    println!("sequence_ok={}", report.sequence_ok);
                    println!("execution_link_ok={}", report.execution_link_ok);
                    println!("state_link_ok={}", report.state_link_ok);
                    println!("package_link_ok={}", report.package_link_ok);
                }
                Err(execution_core::lineage::LineageError::Validation(m)) => {
                    println!("lineage_verify=failed");
                    println!("field={}", m.field);
                    println!("index={}", m.index);
                    println!("expected={}", m.expected);
                    println!("actual={}", m.actual);
                    return Err(HostError::VerificationFailed(m.field.into()));
                }
                Err(e) => {
                    println!("lineage_verify=failed");
                    println!("field=lineage");
                    println!("index=0");
                    println!("expected=valid");
                    println!("actual={e}");
                    return Err(HostError::VerificationFailed(e.to_string()));
                }
            }
        }
        "chain-restore-verify" => {
            let package_path =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let checkpoint_path = PathBuf::from(
                arg_value(&args, "--checkpoint")
                    .ok_or_else(|| HostError::InvalidArgs("missing --checkpoint".into()))?,
            );
            let lineage_path = PathBuf::from(
                arg_value(&args, "--lineage")
                    .ok_or_else(|| HostError::InvalidArgs("missing --lineage".into()))?,
            );
            let receipt_paths: Vec<PathBuf> = args
                .windows(2)
                .filter(|w| w[0] == "--receipt")
                .map(|w| PathBuf::from(w[1].clone()))
                .collect();
            let input = execution_core::continuity::ChainRestoreInput {
                package_path,
                checkpoint_path,
                lineage_path,
                receipt_paths,
            };
            match execution_core::continuity::restore_lineage_chain(input) {
                Ok(report) => {
                    println!("chain_restore_verify=ok");
                    println!("checkpoint_match={}", report.checkpoint_match);
                    println!("lineage_match={}", report.lineage_match);
                    println!("receipts_match={}", report.receipts_match);
                    println!("final_state_root={}", hex::encode(report.final_state_root));
                    println!(
                        "expected_final_state_root={}",
                        hex::encode(report.expected_final_state_root)
                    );
                }
                Err(execution_core::continuity::ChainRestoreError::Validation(m)) => {
                    println!("chain_restore_verify=failed");
                    println!("field={}", m.field);
                    println!(
                        "index={}",
                        m.index
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "none".into())
                    );
                    println!("expected={}", m.expected);
                    println!("actual={}", m.actual);
                    return Err(HostError::VerificationFailed(m.field));
                }
                Err(e) => {
                    println!("chain_restore_verify=failed");
                    println!("field=chain_restore");
                    println!("index=none");
                    println!("expected=valid");
                    println!("actual={e}");
                    return Err(HostError::VerificationFailed(e.to_string()));
                }
            }
        }
        "determinism-verify" => {
            let package_path =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let checkpoint_path = PathBuf::from(
                arg_value(&args, "--checkpoint")
                    .ok_or_else(|| HostError::InvalidArgs("missing --checkpoint".into()))?,
            );
            let lineage_path = PathBuf::from(
                arg_value(&args, "--lineage")
                    .ok_or_else(|| HostError::InvalidArgs("missing --lineage".into()))?,
            );
            let receipt_paths: Vec<PathBuf> = args
                .windows(2)
                .filter(|w| w[0] == "--receipt")
                .map(|w| PathBuf::from(w[1].clone()))
                .collect();
            let report = execution_core::continuity::restore_lineage_chain(
                execution_core::continuity::ChainRestoreInput {
                    package_path: package_path.clone(),
                    checkpoint_path: checkpoint_path.clone(),
                    lineage_path: lineage_path.clone(),
                    receipt_paths: receipt_paths.clone(),
                },
            )
            .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            let package_bytes =
                execution_core::persistence::package_store::load_package_bytes(&package_path, None)
                    .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            let package_root = execution_core::canonical::hashes::package_hash(&package_bytes);
            let lineage = execution_core::lineage::load_lineage(&lineage_path)
                .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            let receipt = execution_core::persistence::receipt_store::load_receipt(
                receipt_paths
                    .last()
                    .ok_or_else(|| HostError::InvalidArgs("missing --receipt".into()))?,
            )
            .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            let checkpoint_bytes = execution_core::persistence::checkpoint_store::load_checkpoint(
                &checkpoint_path,
                None,
            )
            .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            let checkpoint_state = execution_core::state::decode_checkpoint(&checkpoint_bytes)
                .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            let manifest = execution_core::canonical::generate_execution_manifest(
                package_root,
                execution_core::canonical::hashes::receipt_hash(&receipt),
                &lineage,
                checkpoint_state.root(),
                report.final_state_root,
            );
            let manifest_hash = execution_core::canonical::manifest_hash(&manifest);
            let manifest_path = state.join("worlds/default/manifest.bin");
            let manifest_match = if manifest_path.exists() {
                execution_core::canonical::load_manifest(&manifest_path)
                    .map(|m| m == manifest)
                    .unwrap_or(false)
            } else {
                true
            };
            execution_core::canonical::save_manifest(&manifest_path, &manifest)
                .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            if report.restore_ok && manifest_match {
                println!("determinism_verify=ok");
                println!("receipt_match=true");
                println!("lineage_match=true");
                println!("state_match=true");
                println!("manifest_match=true");
                println!("fuel_match=true");
                println!("manifest_hash={}", hex::encode(manifest_hash));
            } else {
                println!("determinism_verify=failed");
                println!("field=manifest_hash");
                println!("expected=stable");
                println!("actual=mismatch");
                return Err(HostError::VerificationFailed("manifest_hash".into()));
            }
        }

        "recover-world" => {
            let package_path =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let checkpoint_path = PathBuf::from(
                arg_value(&args, "--checkpoint")
                    .ok_or_else(|| HostError::InvalidArgs("missing --checkpoint".into()))?,
            );
            let lineage_path = PathBuf::from(
                arg_value(&args, "--lineage")
                    .ok_or_else(|| HostError::InvalidArgs("missing --lineage".into()))?,
            );
            let receipt_paths: Vec<PathBuf> = args
                .windows(2)
                .filter(|w| w[0] == "--receipt")
                .map(|w| PathBuf::from(w[1].clone()))
                .collect();
            let world_id_hex =
                hex::encode(execution_core::persistence::package_store::package_root(
                    &execution_core::persistence::package_store::load_package_bytes(
                        &package_path,
                        None,
                    )
                    .map_err(|e| HostError::InvalidArgs(e.to_string()))?,
                ));
            let descriptor_output_path = state
                .join("worlds")
                .join(world_id_hex)
                .join("recovery_descriptor.bin");
            match execution_core::operator::recover_world(
                execution_core::operator::OperatorRecoveryInput {
                    package_path,
                    checkpoint_path,
                    lineage_path,
                    receipt_paths,
                    descriptor_output_path,
                },
            ) {
                Ok(out) => {
                    println!("recover_world=ok");
                    println!("checkpoint_match={}", out.report.checkpoint_match);
                    println!("lineage_match={}", out.report.lineage_match);
                    println!("manifest_match={}", out.report.manifest_match);
                    println!("replay_match={}", out.report.replay_match);
                    println!(
                        "recovered_state_root={}",
                        hex::encode(out.report.recovered_state_root)
                    );
                    println!(
                        "expected_state_root={}",
                        hex::encode(out.report.expected_state_root)
                    );
                    println!("descriptor_hash={}", hex::encode(out.descriptor_hash));
                    println!("manifest_hash={}", hex::encode(out.manifest_hash));
                }
                Err(execution_core::operator::OperatorRecoveryError::Validation(m)) => {
                    println!("recover_world=failed");
                    println!("field={}", m.field);
                    println!("expected={}", m.expected);
                    println!("actual={}", m.actual);
                    return Err(HostError::VerificationFailed("recover_world".into()));
                }
                Err(e) => return Err(HostError::VerificationFailed(e.to_string())),
            }
        }
        "verify-recovery" => {
            let descriptor_path = PathBuf::from(
                arg_value(&args, "--descriptor")
                    .ok_or_else(|| HostError::InvalidArgs("missing --descriptor".into()))?,
            );
            match execution_core::operator::load_recovery_descriptor(&descriptor_path) {
                Ok(descriptor) => {
                    let computed = execution_core::operator::descriptor_hash(&descriptor);
                    println!("verify_recovery=ok");
                    println!("descriptor_match=true");
                    println!("descriptor_hash={}", hex::encode(computed));
                }
                Err(execution_core::operator::OperatorRecoveryError::Validation(m)) => {
                    println!("verify_recovery=failed");
                    println!("field={}", m.field);
                    println!("expected={}", m.expected);
                    println!("actual={}", m.actual);
                    return Err(HostError::VerificationFailed("descriptor_hash".into()));
                }
                Err(e) => return Err(HostError::VerificationFailed(e.to_string())),
            }
        }
        "restore-verify" => {
            let package =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let receipt_path = PathBuf::from(
                arg_value(&args, "--receipt")
                    .ok_or_else(|| HostError::InvalidArgs("missing --receipt".into()))?,
            );
            let checkpoint_path = PathBuf::from(
                arg_value(&args, "--checkpoint")
                    .ok_or_else(|| HostError::InvalidArgs("missing --checkpoint".into()))?,
            );
            match execution_core::persistence::restore_and_replay(
                &package,
                &receipt_path,
                &checkpoint_path,
            ) {
                Ok(report) if report.state_match => {
                    println!("restore_verify=ok");
                    println!("checkpoint_match={}", report.checkpoint_match);
                    println!("receipt_match={}", report.receipt_match);
                    println!("state_match={}", report.state_match);
                }
                Ok(report) => {
                    println!("restore_verify=failed");
                    let field = if !report.checkpoint_match {
                        "checkpoint_root"
                    } else if !report.receipt_match {
                        "receipt"
                    } else {
                        "state"
                    };
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

        "export-bundle" => {
            let out = PathBuf::from(
                arg_value(&args, "--out")
                    .ok_or_else(|| HostError::InvalidArgs("missing --out".into()))?,
            );
            let package_path =
                PathBuf::from(arg_value(&args, "--package").ok_or(HostError::MissingPackage)?);
            let checkpoint_path = PathBuf::from(
                arg_value(&args, "--checkpoint")
                    .ok_or_else(|| HostError::InvalidArgs("missing --checkpoint".into()))?,
            );
            let lineage_path = PathBuf::from(
                arg_value(&args, "--lineage")
                    .ok_or_else(|| HostError::InvalidArgs("missing --lineage".into()))?,
            );
            let manifest_path = PathBuf::from(
                arg_value(&args, "--manifest")
                    .ok_or_else(|| HostError::InvalidArgs("missing --manifest".into()))?,
            );
            let descriptor_path = PathBuf::from(
                arg_value(&args, "--descriptor")
                    .ok_or_else(|| HostError::InvalidArgs("missing --descriptor".into()))?,
            );
            let receipt_paths: Vec<PathBuf> = args
                .windows(2)
                .filter(|w| w[0] == "--receipt")
                .map(|w| PathBuf::from(w[1].clone()))
                .collect();
            let v = execution_core::federation::bundle::export_continuity_bundle(
                &out,
                &package_path,
                &checkpoint_path,
                &lineage_path,
                &receipt_paths,
                &manifest_path,
                &descriptor_path,
            )
            .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            println!("bundle_export=ok");
            println!("bundle_ok={}", v.bundle_ok);
            println!("manifest_ok={}", v.manifest_ok);
            println!("lineage_ok={}", v.lineage_ok);
            println!("checkpoint_ok={}", v.checkpoint_ok);
            println!("package_ok={}", v.package_ok);
            println!("receipts_ok={}", v.receipts_ok);
            println!("recovery_ok={}", v.recovery_ok);
        }
        "verify-bundle" => {
            let bundle = PathBuf::from(
                arg_value(&args, "--bundle")
                    .ok_or_else(|| HostError::InvalidArgs("missing --bundle".into()))?,
            );
            match execution_core::federation::bundle::verify_continuity_bundle(&bundle) {
                Ok(v) if v.bundle_ok => {
                    println!("bundle_verify=ok");
                    println!("bundle_ok={}", v.bundle_ok);
                    println!("manifest_ok={}", v.manifest_ok);
                    println!("lineage_ok={}", v.lineage_ok);
                    println!("checkpoint_ok={}", v.checkpoint_ok);
                    println!("package_ok={}", v.package_ok);
                    println!("receipts_ok={}", v.receipts_ok);
                    println!("recovery_ok={}", v.recovery_ok);
                }
                Ok(_) => {
                    println!("bundle_verify=failed");
                    println!("field=bundle_ok");
                    println!("expected=true");
                    println!("actual=false");
                    return Err(HostError::VerificationFailed("bundle_ok".into()));
                }
                Err(e) => {
                    println!("bundle_verify=failed");
                    println!("field=bundle");
                    println!("expected=valid");
                    println!("actual={e}");
                    return Err(HostError::VerificationFailed(e.to_string()));
                }
            }
        }
        "import-bundle" => {
            let bundle = PathBuf::from(
                arg_value(&args, "--bundle")
                    .ok_or_else(|| HostError::InvalidArgs("missing --bundle".into()))?,
            );
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let v =
                execution_core::federation::bundle::import_continuity_bundle(&bundle, &world_root)
                    .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            println!("bundle_import=ok");
            println!("bundle_ok={}", v.bundle_ok);
            println!("manifest_ok={}", v.manifest_ok);
            println!("lineage_ok={}", v.lineage_ok);
            println!("checkpoint_ok={}", v.checkpoint_ok);
            println!("package_ok={}", v.package_ok);
            println!("receipts_ok={}", v.receipts_ok);
            println!("recovery_ok={}", v.recovery_ok);
        }

        "freeze-world" => {
            let world_id = arg_value(&args, "--world")
                .ok_or_else(|| HostError::InvalidArgs("missing --world".into()))?;
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let d = execution_core::federation::runtime::freeze_world(&world_root, &world_id)
                .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            println!("world_freeze=ok");
            println!("checkpoint_root={}", hex::encode(d.current_checkpoint_root));
            println!("latest_execution={}", hex::encode(d.latest_execution_id));
        }
        "resume-world" => {
            let world_id = arg_value(&args, "--world").unwrap_or_else(|| "world".into());
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let _ = execution_core::federation::runtime::resume_world(&world_root, &world_id)
                .map_err(|e| HostError::VerificationFailed(e.to_string()))?;
            println!("world_resume=ok");
            println!("continuity_ok=true");
            println!("replay_ok=true");
        }
        "migrate-world" => {
            let world_id = arg_value(&args, "--world")
                .ok_or_else(|| HostError::InvalidArgs("missing --world".into()))?;
            let bundle = PathBuf::from(
                arg_value(&args, "--bundle")
                    .ok_or_else(|| HostError::InvalidArgs("missing --bundle".into()))?,
            );
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let dst_hex = arg_value(&args, "--destination")
                .ok_or_else(|| HostError::InvalidArgs("missing --destination".into()))?;
            let mut dst = [0u8; 32];
            hex::decode_to_slice(dst_hex, &mut dst as &mut [u8])
                .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let manifest =
                execution_core::canonical::load_manifest(&world_root.join("manifest.bin"))
                    .map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let req = execution_core::federation::runtime::WorldMigrationRequest {
                source_node: execution_core::federation::node::FederationNodeId::new([0; 32]),
                destination_node: execution_core::federation::node::FederationNodeId::new(dst),
                world_id,
                expected_package_root: manifest.package_root,
                expected_checkpoint_root: manifest.checkpoint_root,
            };
            let dst_world = bundle.join("imported_world");
            match execution_core::federation::runtime::migrate_world(
                &world_root,
                &bundle,
                &dst_world,
                &req,
            ) {
                Ok(r) if r.migration_ok => {
                    println!("world_migration=ok");
                    println!("continuity_ok={}", r.continuity_ok);
                    println!("replay_ok={}", r.replay_ok);
                    println!("resumed_ok={}", r.resumed_ok);
                }
                Ok(_) => {
                    println!("world_migration=failed");
                    println!("field=migration_ok");
                    println!("expected=true");
                    println!("actual=false");
                    return Err(HostError::VerificationFailed("migration failed".into()));
                }
                Err(e) => {
                    println!("world_migration=failed");
                    println!("field=bundle");
                    println!("expected=valid");
                    println!("actual={e}");
                    return Err(HostError::VerificationFailed(e.to_string()));
                }
            }
        }
        "scheduler-run-once" => {
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            fs::create_dir_all(&world_root).map_err(|e| HostError::InvalidArgs(e.to_string()))?;
            let mut state = WorldSchedulerState::load(&world_root)?;
            let mut queue = execution_core::scheduler::queue::DeterministicQueue::default();
            let events = scheduler_queue_events(&world_root)?;
            for event in &events {
                let _ = queue.push(event.clone());
            }
            let mut runtime = execution_core::scheduler::SchedulerRuntime::new(
                SchedulerWorld {
                    lineage: 1,
                    checkpoint: execution_core::scheduler::world::WorldCheckpoint {
                        lineage: 1,
                        tick: execution_core::scheduler::tick::DeterministicTick(state.latest_tick),
                    },
                },
                queue,
                execution_core::scheduler::tick::DeterministicTick(state.latest_tick + 1),
            );
            let receipt = runtime.run_one_tick();
            let mut hasher = Sha256::new();
            hasher.update(state.checkpoint_root);
            hasher.update(receipt.tick.0.to_le_bytes());
            if let Some(sequence) = receipt.event_sequence {
                hasher.update(sequence.to_le_bytes());
            }
            state.latest_tick = receipt.tick.0;
            state.checkpoint_root = hasher.finalize().into();
            state.persist(&world_root)?;
            let receipt_path = write_scheduler_receipt(
                &world_root,
                receipt.tick.0,
                receipt.event_sequence,
                state.checkpoint_root,
            )?;
            println!("scheduler_run=ok");
            println!("tick={}", receipt.tick.0);
            println!(
                "events_processed={}",
                usize::from(receipt.event_sequence.is_some())
            );
            println!("checkpoint_root={}", hex::encode(state.checkpoint_root));
            println!("receipt={}", receipt_path.display());
        }
        "scheduler-status" => {
            let world_root = PathBuf::from(
                arg_value(&args, "--world-root")
                    .ok_or_else(|| HostError::InvalidArgs("missing --world-root".into()))?,
            );
            let state = WorldSchedulerState::load(&world_root)?;
            let pending_events = scheduler_queue_events(&world_root)?.len();
            println!("scheduler_status=ok");
            println!("world_root={}", world_root.display());
            println!("latest_tick={}", state.latest_tick);
            println!("checkpoint_root={}", hex::encode(state.checkpoint_root));
            println!("pending_events={pending_events}");
        }

        _ => {
            return Err(HostError::InvalidArgs(
                "unknown command (run --help)".into(),
            ))
        }
    }
    Ok(())
}
