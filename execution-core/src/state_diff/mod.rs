use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::hashing::sha256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalStateDiff {
    pub updates: BTreeMap<String, String>,
}

impl CanonicalStateDiff {
    pub fn canonical_hash(&self) -> anyhow::Result<String> {
        let bytes = bincode::serialize(self)?;
        Ok(hex::encode(sha256(&bytes)))
    }
}
