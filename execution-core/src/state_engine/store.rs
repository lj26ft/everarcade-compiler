use std::collections::BTreeMap;

use everarcade_abi::StateChange;

use super::{merkle, snapshot::StateSnapshot};

#[derive(Debug, Clone, Default)]
pub struct StateStore {
    entries: BTreeMap<String, String>,
}

impl StateStore {
    pub fn new(entries: BTreeMap<String, String>) -> Self {
        Self { entries }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }
    pub fn set(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }
    pub fn apply_changes(&mut self, changes: &[StateChange]) {
        for c in changes {
            self.entries.insert(c.key.clone(), c.after.clone());
        }
    }
    pub fn snapshot(&self, previous_snapshot_hash: Option<String>) -> StateSnapshot {
        StateSnapshot::new(self.entries.clone(), previous_snapshot_hash)
    }
    pub fn root(&self) -> [u8; 32] {
        let ordered: Vec<(String, String)> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        merkle::compute_state_root(&ordered)
    }
    pub fn into_state(self) -> BTreeMap<String, String> {
        self.entries
    }
}
