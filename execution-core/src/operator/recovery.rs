use std::path::PathBuf;

use crate::{
    canonical::{generate_execution_manifest, hashes, manifest_hash},
    continuity::{restore_lineage_chain, ChainRestoreInput, ChainRestoreError},
    lineage,
    persistence::{checkpoint_store, package_store, receipt_store},
};

use super::{
    continuity::{descriptor_hash, OperatorRecoveryMismatch, OperatorRecoveryReport, WorldRecoveryDescriptor},
    errors::OperatorRecoveryError,
    registry::save_recovery_descriptor,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorRecoveryInput {
    pub package_path: PathBuf,
    pub checkpoint_path: PathBuf,
    pub lineage_path: PathBuf,
    pub receipt_paths: Vec<PathBuf>,
    pub descriptor_output_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorRecoveryOutput {
    pub report: OperatorRecoveryReport,
    pub descriptor: WorldRecoveryDescriptor,
    pub descriptor_hash: [u8; 32],
    pub manifest_hash: [u8; 32],
}

pub fn recover_world(input: OperatorRecoveryInput) -> Result<OperatorRecoveryOutput, OperatorRecoveryError> {
    let restore = restore_lineage_chain(ChainRestoreInput {
        package_path: input.package_path.clone(),
        checkpoint_path: input.checkpoint_path.clone(),
        lineage_path: input.lineage_path.clone(),
        receipt_paths: input.receipt_paths.clone(),
    }).map_err(map_chain_err)?;

    let package_bytes = package_store::load_package_bytes(&input.package_path, None)
        .map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    let package_root = hashes::package_hash(&package_bytes);
    let lineage_chain = lineage::load_lineage(&input.lineage_path)
        .map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    let checkpoint_bytes = checkpoint_store::load_checkpoint(&input.checkpoint_path, None)
        .map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    let checkpoint_state = crate::state::decode_checkpoint(&checkpoint_bytes)
        .map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    let latest_receipt = receipt_store::load_receipt(input.receipt_paths.last().ok_or_else(|| OperatorRecoveryError::Storage("missing receipt".into()))?)
        .map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;

    let manifest = generate_execution_manifest(
        package_root,
        hashes::receipt_hash(&latest_receipt),
        &lineage_chain,
        checkpoint_state.root(),
        restore.final_state_root,
    );
    let manifest_h = manifest_hash(&manifest);
    let latest_execution_id = lineage_chain.records.last().map(|r| r.execution_id).unwrap_or([0u8;32]);
    let descriptor = WorldRecoveryDescriptor {
        world_id: package_root,
        package_root,
        latest_checkpoint_root: checkpoint_state.root(),
        latest_execution_id,
        manifest_hash: manifest_h,
    };
    save_recovery_descriptor(&input.descriptor_output_path, &descriptor)?;

    Ok(OperatorRecoveryOutput {
        report: OperatorRecoveryReport {
            recovery_ok: restore.restore_ok,
            checkpoint_match: restore.checkpoint_match,
            lineage_match: restore.lineage_match,
            manifest_match: true,
            replay_match: restore.receipts_match,
            recovered_state_root: restore.final_state_root,
            expected_state_root: restore.expected_final_state_root,
        },
        descriptor_hash: descriptor_hash(&descriptor),
        descriptor,
        manifest_hash: manifest_h,
    })
}

fn map_chain_err(err: ChainRestoreError) -> OperatorRecoveryError {
    match err {
        ChainRestoreError::Validation(m) => OperatorRecoveryError::Validation(OperatorRecoveryMismatch {
            field: m.field,
            expected: m.expected,
            actual: m.actual,
        }),
        other => OperatorRecoveryError::Storage(other.to_string()),
    }
}
