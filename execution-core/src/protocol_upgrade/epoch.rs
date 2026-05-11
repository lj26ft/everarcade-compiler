use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolEpoch {
    pub epoch_id: u64,
    pub abi_version: String,
    pub hash_version: String,
    pub receipt_version: String,
    pub snapshot_version: String,
    pub dag_version: String,
    pub execution_version: String,
}
