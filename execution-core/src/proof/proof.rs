use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionProof {
    pub proof_system: String,
    pub execution_root: String,
    pub receipt_hash: String,
    pub snapshot_hash: String,
    pub epoch_id: u64,
    pub proof_bytes: Vec<u8>,
}
