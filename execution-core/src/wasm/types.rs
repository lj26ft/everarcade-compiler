use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::hashing::sha256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmExecutionInput {
    pub module_hash: String,
    pub input_hash: String,
    pub fuel_limit: u64,
    pub initial_state_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmStateDiff {
    pub updates: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmExecutionError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmHostCallTrace {
    pub seq: u64,
    pub call: String,
    pub payload_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmExecutionOutput {
    pub final_state_root: String,
    pub state_diff: WasmStateDiff,
    pub host_calls: Vec<WasmHostCallTrace>,
    pub error: Option<WasmExecutionError>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WasmExecutionManifest {
    pub module_hash: String,
    pub input_hash: String,
    pub host_abi_version: String,
    pub fuel_limit: u64,
    pub deterministic_engine_config_hash: String,
    pub initial_state_root: String,
    pub final_state_root: String,
    pub receipt_hash: String,
    pub state_diff_hash: String,
    pub journal_hash: String,
}

impl WasmExecutionManifest {
    pub fn canonical_hash(&self) -> anyhow::Result<String> {
        let bytes = bincode::serialize(self)?;
        Ok(hex::encode(sha256(&bytes)))
    }
}
