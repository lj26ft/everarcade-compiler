use crate::{hashing::sha256, State, StateChange};
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmExecutionReceipt {
    pub execution_id: Hash256,
    pub pre_state_root: Hash256,
    pub post_state_root: Hash256,
    pub input_hash: Hash256,
    pub output_hash: Hash256,
    pub diff_hash: Hash256,
    pub events_hash: Hash256,
    pub fuel_used: u64,
    pub wasm_hash: Hash256,
}

pub fn state_root(state: &State) -> Hash256 {
    let entries: Vec<(&String, &String)> = state.iter().collect();
    let encoded = bincode::serialize(&entries).unwrap_or_default();
    sha256(&encoded)
}

pub fn events_hash(output: &[u8]) -> Hash256 {
    sha256(output)
}

pub fn execution_id(input_hash: Hash256, wasm_hash: Hash256, post_state_root: Hash256) -> Hash256 {
    sha256(
        &[
            input_hash.as_slice(),
            wasm_hash.as_slice(),
            post_state_root.as_slice(),
        ]
        .concat(),
    )
}

pub fn compute_diff_hash(diff: &[StateChange]) -> Hash256 {
    let mut canonical = diff.to_vec();
    canonical.sort_by(|a, b| a.key.cmp(&b.key));
    let encoded = bincode::serialize(&canonical).unwrap_or_default();
    sha256(&encoded)
}
