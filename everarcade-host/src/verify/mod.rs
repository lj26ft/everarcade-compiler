pub mod anchor_verify;
pub mod checkpoint_verify;
pub mod package_verify;
pub mod receipt_verify;
pub mod verification_report;

use std::path::Path;

use crate::{error::HostError, state_folder::node_manifest::read_node_manifest};

pub use verification_report::VerificationReport;

pub fn verify_state(state: &Path) -> Result<VerificationReport, HostError> {
    let manifest = read_node_manifest(state)?;
    let package_valid = package_verify::verify_package_artifacts(state);
    let receipt_valid = receipt_verify::verify_receipt(state, &manifest)?;
    let checkpoint_valid = checkpoint_verify::verify_checkpoint(state, &manifest)?;
    let anchor_valid = anchor_verify::verify_anchor(state, &manifest)?;
    Ok(VerificationReport {
        package_valid,
        receipt_valid,
        checkpoint_valid,
        anchor_valid,
    })
}
