use crate::hashing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceNode {
    pub node_id: String,
    pub contract_hash: String,
    pub fuel_used: u64,
    pub memory_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceTransition {
    pub key: String,
    pub old_value: Option<String>,
    pub new_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionTrace {
    pub epoch_id: u64,
    pub execution_root: String,
    pub snapshot_hash: String,
    pub nodes: Vec<TraceNode>,
    pub transitions: Vec<TraceTransition>,
}

impl ExecutionTrace {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("trace serialize failed")
    }

    pub fn digest(&self) -> String {
        hashing::hash_bytes(&self.canonical_bytes())
    }
}
