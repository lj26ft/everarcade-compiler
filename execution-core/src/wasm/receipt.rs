use crate::{hashing::sha256, StateChange};
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmExecutionReceipt {
    pub input_hash: Hash256,
    pub output_hash: Hash256,
    pub diff_hash: Hash256,
    pub fuel_used: u64,
    pub wasm_hash: Hash256,
}

pub fn compute_diff_hash(diff: &[StateChange]) -> Hash256 {
    let mut canonical = diff.to_vec();
    canonical.sort_by(|a, b| a.key.cmp(&b.key));
    let encoded = bincode::serialize(&canonical).unwrap_or_default();
    sha256(&encoded)
}
