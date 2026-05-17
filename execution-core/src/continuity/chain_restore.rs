use std::path::PathBuf;

use crate::{
    lineage,
    persistence::{checkpoint_store, package_store, receipt_store},
    state::{apply_diff, decode_checkpoint, CanonicalState},
    vm::{execute_vm_boundary, validate_vm_receipt, Hash, VmExecutionInput},
};

use super::errors::ChainRestoreError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainRestoreInput {
    pub package_path: PathBuf,
    pub checkpoint_path: PathBuf,
    pub lineage_path: PathBuf,
    pub receipt_paths: Vec<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainRestoreReport {
    pub restore_ok: bool,
    pub checkpoint_match: bool,
    pub lineage_match: bool,
    pub receipts_match: bool,
    pub final_state_root: Hash,
    pub expected_final_state_root: Hash,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainRestoreMismatch {
    pub field: String,
    pub index: Option<usize>,
    pub expected: String,
    pub actual: String,
}

pub fn restore_lineage_chain(
    input: ChainRestoreInput,
) -> Result<ChainRestoreReport, ChainRestoreError> {
    let package_bytes = package_store::load_package_bytes(&input.package_path, None)?;
    let package_root = package_store::package_root(&package_bytes);
    let checkpoint_bytes = checkpoint_store::load_checkpoint(&input.checkpoint_path, None)?;
    let mut state: CanonicalState = decode_checkpoint(&checkpoint_bytes).map_err(|e| {
        ChainRestoreError::Validation(ChainRestoreMismatch {
            field: "checkpoint_decode".into(),
            index: None,
            expected: "canonical_state".into(),
            actual: e.to_string(),
        })
    })?;
    let checkpoint_root = state.root();

    let chain = lineage::load_lineage(&input.lineage_path)?;
    lineage::validate_lineage_chain(&chain)?;

    if chain.records.is_empty() {
        return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
            field: "lineage_records".into(),
            index: None,
            expected: ">=1".into(),
            actual: "0".into(),
        }));
    }
    if chain.package_root != package_root {
        return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
            field: "package_root".into(),
            index: None,
            expected: hex::encode(chain.package_root),
            actual: hex::encode(package_root),
        }));
    }
    if chain.records.len() != input.receipt_paths.len() {
        return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
            field: "receipt_count".into(),
            index: None,
            expected: chain.records.len().to_string(),
            actual: input.receipt_paths.len().to_string(),
        }));
    }
    if chain.records[0].pre_state_root != checkpoint_root {
        return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
            field: "checkpoint_root".into(),
            index: Some(0),
            expected: hex::encode(chain.records[0].pre_state_root),
            actual: hex::encode(checkpoint_root),
        }));
    }

    let mut current_state_root = checkpoint_root;
    for (idx, receipt_path) in input.receipt_paths.iter().enumerate() {
        let receipt = receipt_store::load_receipt(receipt_path)?;
        let record = &chain.records[idx];

        if record.execution_id != receipt.execution_root {
            return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
                field: "execution_id".into(),
                index: Some(idx),
                expected: hex::encode(record.execution_id),
                actual: hex::encode(receipt.execution_root),
            }));
        }
        if receipt.prior_replay_root != current_state_root {
            return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
                field: "pre_state_root".into(),
                index: Some(idx),
                expected: hex::encode(current_state_root),
                actual: hex::encode(receipt.prior_replay_root),
            }));
        }

        let replay_input = VmExecutionInput {
            package_manifest_root: package_root,
            civilization_root: package_root,
            replay_root: receipt.prior_replay_root,
            checkpoint_root: receipt.checkpoint_root,
            payload_root: receipt.checkpoint_root,
        };
        let (replayed_receipt, _) = execute_vm_boundary(&replay_input);
        if !validate_vm_receipt(&receipt) || replayed_receipt != receipt {
            return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
                field: "receipt".into(),
                index: Some(idx),
                expected: hex::encode(replayed_receipt.receipt_id),
                actual: hex::encode(receipt.receipt_id),
            }));
        }

        current_state_root = apply_diff(&mut state, &replayed_receipt.state_diff).map_err(|e| {
            ChainRestoreError::Validation(ChainRestoreMismatch {
                field: "state_before".into(),
                index: Some(idx),
                expected: "diff applies".into(),
                actual: e.to_string(),
            })
        })?;
        if record.post_state_root != current_state_root {
            return Err(ChainRestoreError::Validation(ChainRestoreMismatch {
                field: "post_state_root".into(),
                index: Some(idx),
                expected: hex::encode(record.post_state_root),
                actual: hex::encode(current_state_root),
            }));
        }
    }

    Ok(ChainRestoreReport {
        restore_ok: true,
        checkpoint_match: true,
        lineage_match: true,
        receipts_match: true,
        final_state_root: current_state_root,
        expected_final_state_root: chain
            .records
            .last()
            .map(|r| r.post_state_root)
            .unwrap_or(checkpoint_root),
    })
}
