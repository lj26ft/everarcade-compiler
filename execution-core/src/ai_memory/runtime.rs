use serde::{Deserialize, Serialize};

use super::{
    store::{AiMemoryEntry, AiMemoryStore},
    validation,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AiMemoryError {
    NonAppendOnlyMutation,
    InvalidReplayRoot,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiMemoryRuntime {
    pub store: AiMemoryStore,
}

impl AiMemoryRuntime {
    pub fn append(
        &mut self,
        entity_id: &str,
        fact: &str,
        replay_root: &str,
    ) -> Result<AiMemoryEntry, AiMemoryError> {
        if replay_root.is_empty() {
            return Err(AiMemoryError::InvalidReplayRoot);
        }
        let sequence = self
            .store
            .entries
            .iter()
            .filter(|e| e.entity_id == entity_id)
            .count() as u64;
        let entry = AiMemoryEntry {
            entity_id: entity_id.to_string(),
            sequence,
            fact: fact.to_string(),
            replay_root: replay_root.to_string(),
        };
        self.store.entries.push(entry.clone());
        Ok(entry)
    }
    pub fn replace_at(&mut self, index: usize, entry: AiMemoryEntry) -> Result<(), AiMemoryError> {
        if index < self.store.entries.len() && self.store.entries[index] != entry {
            return Err(AiMemoryError::NonAppendOnlyMutation);
        }
        self.store.entries.push(entry);
        Ok(())
    }
    pub fn validate(&self) -> bool {
        validation::memory_is_append_only(&self.store)
    }
}
