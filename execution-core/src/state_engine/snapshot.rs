use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use super::merkle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    pub state_root: String,
    pub state_entries: BTreeMap<String, String>,
    pub snapshot_hash: String,
    pub previous_snapshot_hash: Option<String>,
}

impl StateSnapshot {
    pub fn new(state_entries: BTreeMap<String, String>, previous_snapshot_hash: Option<String>) -> Self {
        let ordered: Vec<(String, String)> = state_entries.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let state_root = merkle::to_hex(&merkle::compute_state_root(&ordered));
        let mut snapshot = Self {
            state_root,
            state_entries,
            snapshot_hash: String::new(),
            previous_snapshot_hash,
        };
        snapshot.snapshot_hash = snapshot.hash();
        snapshot
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.state_root.as_bytes());
        for (k, v) in &self.state_entries {
            hasher.update((k.len() as u64).to_le_bytes());
            hasher.update(k.as_bytes());
            hasher.update((v.len() as u64).to_le_bytes());
            hasher.update(v.as_bytes());
        }
        if let Some(prev) = &self.previous_snapshot_hash {
            hasher.update(prev.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
}
