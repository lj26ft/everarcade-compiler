use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const ABI_VERSION: &str = "everarcade-execution-abi-v2";

pub type State = BTreeMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub id: String,
    pub action: String,
    pub payload: serde_json::Value,
    pub deps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub nodes: Vec<ExecutionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInput {
    pub state: State,
    pub plan: ExecutionPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub abi_version: String,
    pub contract_hash: String,
    pub input_hash: String,
    pub previous_state_root: String,
    pub new_state_root: String,
    pub execution_root: String,
    pub fuel_used: u64,
    pub memory_used: u64,
    pub node_hashes: BTreeMap<String, String>,
    pub state_changes: Vec<StateChange>,
    pub output_hash: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmOutput {
    pub updated_state: State,
    pub receipt: ExecutionReceipt,
}

pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, bincode::Error> { bincode::serialize(value) }
pub fn deserialize<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, bincode::Error> { bincode::deserialize(bytes) }
