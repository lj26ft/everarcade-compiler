use std::fs;

use execution_core::{
    external::anchor_emission::emit_external_anchor_receipt,
    vm::{execute_vm_boundary, validate_vm_receipt, VmExecutionInput, VmExecutionReceipt},
};
use sha2::{Digest, Sha256};

use crate::{
    anchor_queue::{queue_anchor_intent, AnchorQueueItem},
    checkpoint_store::write_checkpoint_root,
    config::HostConfig,
    error::HostError,
    evernode::{
        anchor_intent::EvernodeAnchorIntent, host_manifest::HostManifest,
        instance::compute_vm_instance_root,
    },
    package_loader::load_package,
    persistence::HostPaths,
    receipt_store::write_receipt,
    xrpl::transaction_builder::build_anchor_intent,
};

pub struct HostRunResult {
    pub receipt: VmExecutionReceipt,
}

pub fn run_package_once(config: HostConfig) -> Result<HostRunResult, HostError> {
    let package = load_package(&config.package_path)?;
    let paths = HostPaths::new(config.data_dir);
    paths.ensure()?;
    let input = VmExecutionInput {
        package_manifest_root: package.execution_root,
        civilization_root: package.execution_root,
        replay_root: package.replay_root,
        checkpoint_root: package.checkpoint_root,
        payload_root: package.proof_root,
    };
    let (receipt, _) = execute_vm_boundary(&input);
    if !validate_vm_receipt(&receipt) {
        return Err(HostError::InvalidReceipt);
    }
    let _receipt_path = write_receipt(&paths.receipts, &receipt)?;
    let _checkpoint_path = write_checkpoint_root(&paths.checkpoints, &receipt.checkpoint_root)?;
    let external = emit_external_anchor_receipt(receipt.anchor_root);
    let xrpl_intent = build_anchor_intent(
        receipt.receipt_id,
        external.xrpl_anchor_root.unwrap_or(receipt.anchor_root),
    );

    let vm_instance = compute_vm_instance_root(receipt.package_root, receipt.receipt_id);
    let manifest = HostManifest {
        vm_instance_root_hex: hex::encode(vm_instance),
        package_root_hex: hex::encode(receipt.package_root),
        receipt_root_hex: hex::encode(receipt.receipt_id),
        checkpoint_root_hex: hex::encode(receipt.checkpoint_root),
    };
    let manifest_bytes = serde_json::to_vec(&manifest).expect("manifest serialization");
    let manifest_hash: [u8; 32] = Sha256::digest(manifest_bytes).into();
    let evernode_intent = EvernodeAnchorIntent {
        receipt_id_hex: hex::encode(receipt.receipt_id),
        manifest_hash_hex: hex::encode(manifest_hash),
    };
    fs::write(
        paths
            .manifests
            .join(format!("{}.json", hex::encode(receipt.receipt_id))),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )?;
    queue_anchor_intent(
        &paths.anchors,
        &AnchorQueueItem {
            receipt_id_hex: hex::encode(receipt.receipt_id),
            xrpl_intent: serde_json::to_value(xrpl_intent).unwrap(),
            evernode_intent: serde_json::to_value(evernode_intent).unwrap(),
        },
    )?;
    Ok(HostRunResult { receipt })
}
