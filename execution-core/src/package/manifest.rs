use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionManifest {
    pub package_id: String,
    pub protocol_epoch: u64,
    pub abi_version: String,
    pub snapshot_version: String,
    pub execution_root: String,
    pub state_root: String,
    pub contract_hashes: Vec<String>,
    pub package_hash: String,
}
