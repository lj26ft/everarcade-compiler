use std::{fs, path::PathBuf};

use everarcade_host::{
    config::HostConfig,
    error::HostError,
    persistence::HostPaths,
    receipt_store::read_receipt,
    run_package_once,
    state_folder::{
        node_manifest::{read_node_manifest, write_node_manifest, NodeManifest},
        validation::validate,
    },
};
use execution_core::vm::validate_vm_receipt;

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
            manifest.last_anchor_root = Some(hex::encode(result.receipt.anchor_root));
            manifest.state_root = hex::encode(result.receipt.checkpoint_root);
            write_node_manifest(&state, &manifest)?;
            println!("receipt={}", hex::encode(result.receipt.receipt_id));
        }
        "verify" => {
            let manifest = read_node_manifest(&state)?;
            let rid = manifest
                .last_receipt_root
                .ok_or(HostError::InvalidReceipt)?;
            let receipt = read_receipt(&state.join("receipts").join(format!("{rid}.bin")))?;
            if !validate_vm_receipt(&receipt) {
                return Err(HostError::VerificationFailed("receipt invalid".into()));
            }
            let cp = manifest
                .last_checkpoint_root
                .ok_or(HostError::InvalidCheckpoint)?;
            if !state.join("checkpoints").join(format!("{cp}.bin")).exists() {
                return Err(HostError::InvalidCheckpoint);
            }
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
        }
        "anchor-intent" => {
            let manifest = read_node_manifest(&state)?;
            let rid = manifest
                .last_receipt_root
                .ok_or(HostError::AnchorIntentMissing)?;
            let p = state.join("anchors").join(format!("{rid}.json"));
            if !p.exists() {
                return Err(HostError::AnchorIntentMissing);
            }
            println!("{}", p.display());
        }
        _ => {
            return Err(HostError::InvalidArgs(
                "usage: init|run|verify|status|anchor-intent".into(),
            ))
        }
    }
    Ok(())
}
