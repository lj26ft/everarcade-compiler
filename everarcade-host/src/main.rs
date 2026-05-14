use std::{fs, path::PathBuf};

use everarcade_host::{
    config::HostConfig,
    error::HostError,
    fixture::generate_fixture_to_path,
    persistence::HostPaths,
    run_package_once,
    state_folder::{
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
        _ => {
            return Err(HostError::InvalidArgs(
                "usage: init|generate-fixture|run|verify|status|anchor-intent".into(),
            ))
        }
    }
    Ok(())
}
