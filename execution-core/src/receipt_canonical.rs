use crate::hashing::sha256;
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalExecutionReceipt {
    pub execution_id: Hash256,
    pub module_hash: Hash256,
    pub input_hash: Hash256,
    pub fuel_consumed: u64,
    pub state_root_before: Hash256,
    pub state_root_after: Hash256,
    pub state_diff_hash: Hash256,
    pub replay_hash: Hash256,
    pub exit_code: i32,
}

impl CanonicalExecutionReceipt {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("receipt serialization failed")
    }

    pub fn canonical_hash(&self) -> Hash256 {
        sha256(&self.canonical_bytes())
    }
}
