use std::path::Path;

use execution_core::vm::validate_vm_receipt;

use crate::{
    error::HostError, receipt_store::read_receipt, state_folder::node_manifest::NodeManifest,
};

pub fn verify_receipt(state: &Path, manifest: &NodeManifest) -> Result<bool, HostError> {
    let rid = manifest
        .last_receipt_root
        .as_ref()
        .ok_or(HostError::InvalidReceipt)?;
    let receipt = read_receipt(&state.join("receipts").join(format!("{rid}.bin")))?;
    Ok(validate_vm_receipt(&receipt))
}
