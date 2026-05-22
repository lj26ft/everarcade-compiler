use crate::hashing::sha256;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalExecutionState {
    pub revision: u64,
    pub entries: BTreeMap<String, Vec<u8>>,
}

impl CanonicalExecutionState {
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("canonical state serialization failed")
    }

    pub fn to_canonical_json(&self) -> String {
        serde_json::to_string(self).expect("canonical state json serialization failed")
    }

    pub fn canonical_hash(&self) -> [u8; 32] {
        sha256(&self.to_canonical_bytes())
    }
}
