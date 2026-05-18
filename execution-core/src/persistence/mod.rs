pub mod checkpoint_store;
pub mod errors;
pub mod layout;
pub mod package_store;
pub mod receipt_store;

use std::path::Path;

use crate::vm::{execute_vm_boundary, validate_vm_receipt, VmExecutionInput};

pub use errors::PersistenceError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayVerification {
    pub checkpoint_match: bool,
    pub receipt_match: bool,
    pub state_match: bool,
}

pub fn restore_and_replay(
    package_path: &Path,
    receipt_path: &Path,
    checkpoint_path: &Path,
) -> Result<ReplayVerification, PersistenceError> {
    let package = package_store::load_package_bytes(package_path, None)?;
    let receipt = receipt_store::load_receipt(receipt_path)?;
    let checkpoint =
        checkpoint_store::load_checkpoint(checkpoint_path, Some(receipt.checkpoint_root))?;

    let package_match = package_store::package_root(&package) == receipt.package_root;
    let checkpoint_match =
        checkpoint_store::checkpoint_root(&checkpoint) == receipt.checkpoint_root;

    let input = VmExecutionInput {
        package_manifest_root: receipt.package_root,
        civilization_root: receipt.package_root,
        pre_state_root: receipt.prior_replay_root,
        prior_replay_root_value: receipt.prior_replay_root,
        checkpoint_root: receipt.checkpoint_root,
        payload_root: receipt.checkpoint_root,
    };
    let (replayed, _) = execute_vm_boundary(&input);
    let receipt_match = validate_vm_receipt(&receipt) && replayed == receipt;

    Ok(ReplayVerification {
        checkpoint_match,
        receipt_match,
        state_match: package_match && checkpoint_match && receipt_match,
    })
}
