use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiMemoryEntry {
    pub entity_id: String,
    pub sequence: u64,
    pub fact: String,
    pub replay_root: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiMemoryStore {
    pub entries: Vec<AiMemoryEntry>,
}

impl AiMemoryStore {
    pub fn entries_for(&self, entity_id: &str) -> Vec<AiMemoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.entity_id == entity_id)
            .cloned()
            .collect()
    }
}
