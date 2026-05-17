use serde::{Deserialize, Serialize};

use crate::{hashing::sha256, wasm::receipt::Hash256};

use super::errors::{LineageError, LineageMismatch};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLineageRecord {
    pub sequence: u64,
    pub previous_execution_id: Option<Hash256>,
    pub execution_id: Hash256,
    pub pre_state_root: Hash256,
    pub post_state_root: Hash256,
    pub receipt_hash: Hash256,
    pub package_root: Hash256,
}

impl ExecutionLineageRecord {
    pub fn canonical_hash(&self) -> Result<Hash256, LineageError> {
        let encoded = bincode::serialize(self).map_err(LineageError::Encode)?;
        Ok(sha256(&encoded))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLineageChain {
    pub world_id: Hash256,
    pub package_root: Hash256,
    pub records: Vec<ExecutionLineageRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineageValidation {
    pub lineage_ok: bool,
    pub sequence_ok: bool,
    pub execution_link_ok: bool,
    pub state_link_ok: bool,
    pub package_link_ok: bool,
}

pub fn validate_lineage_chain(
    chain: &ExecutionLineageChain,
) -> Result<LineageValidation, LineageError> {
    for (idx, record) in chain.records.iter().enumerate() {
        if record.package_root != chain.package_root {
            return Err(LineageError::Validation(LineageMismatch {
                field: "package_root",
                index: idx,
                expected: hex::encode(chain.package_root),
                actual: hex::encode(record.package_root),
            }));
        }
        if idx == 0 {
            if record.previous_execution_id.is_some() {
                return Err(LineageError::Validation(LineageMismatch {
                    field: "previous_execution_id",
                    index: idx,
                    expected: "none".into(),
                    actual: hex::encode(record.previous_execution_id.unwrap()),
                }));
            }
            continue;
        }

        let prev = &chain.records[idx - 1];
        if record.sequence != prev.sequence + 1 {
            return Err(LineageError::Validation(LineageMismatch {
                field: "sequence",
                index: idx,
                expected: (prev.sequence + 1).to_string(),
                actual: record.sequence.to_string(),
            }));
        }
        if record.previous_execution_id != Some(prev.execution_id) {
            return Err(LineageError::Validation(LineageMismatch {
                field: "previous_execution_id",
                index: idx,
                expected: hex::encode(prev.execution_id),
                actual: hex::encode(record.previous_execution_id.unwrap_or([0u8; 32])),
            }));
        }
        if record.pre_state_root != prev.post_state_root {
            return Err(LineageError::Validation(LineageMismatch {
                field: "pre_state_root",
                index: idx,
                expected: hex::encode(prev.post_state_root),
                actual: hex::encode(record.pre_state_root),
            }));
        }
    }

    Ok(LineageValidation {
        lineage_ok: true,
        sequence_ok: true,
        execution_link_ok: true,
        state_link_ok: true,
        package_link_ok: true,
    })
}
