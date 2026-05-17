use std::path::Path;

use execution_core::vm::{
    execute_vm_boundary, validate_vm_receipt, VmExecutionInput, VmExecutionReceipt,
};

use crate::{error::HostError, package_loader::load_package, receipt_store::read_receipt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayVerificationReport {
    pub receipt_canonical_valid: bool,
    pub package_matches_receipt: bool,
    pub deterministic_replay_match: bool,
}

impl ReplayVerificationReport {
    pub fn verified(&self) -> bool {
        self.receipt_canonical_valid
            && self.package_matches_receipt
            && self.deterministic_replay_match
    }
}

pub fn verify_receipt_replay(
    package_path: &Path,
    receipt_path: &Path,
) -> Result<ReplayVerificationReport, HostError> {
    let package = load_package(package_path)?;
    let receipt = read_receipt(receipt_path)?;
    verify_receipt_replay_from_artifacts(&package, &receipt)
}

pub fn verify_receipt_replay_from_artifacts(
    package: &execution_core::civilization::CivilizationPackage,
    receipt: &VmExecutionReceipt,
) -> Result<ReplayVerificationReport, HostError> {
    let receipt_canonical_valid = validate_vm_receipt(receipt);
    let package_matches_receipt = package.execution_root == receipt.package_root;

    let input = VmExecutionInput {
        package_manifest_root: package.execution_root,
        civilization_root: package.execution_root,
        replay_root: receipt.prior_replay_root,
        checkpoint_root: receipt.checkpoint_root,
        payload_root: package.proof_root,
    };
    let (replayed_receipt, _) = execute_vm_boundary(&input);
    let deterministic_replay_match = &replayed_receipt == receipt;

    Ok(ReplayVerificationReport {
        receipt_canonical_valid,
        package_matches_receipt,
        deterministic_replay_match,
    })
}
