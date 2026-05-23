use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LifecycleTransitionKind {
    Bootstrap,
    Activation,
    Upgrade,
    Migration,
    Pause,
    Recovery,
    Archival,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifecycleTransition {
    pub sequence: u64,
    pub kind: LifecycleTransitionKind,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LifecycleJournal {
    pub transitions: Vec<LifecycleTransition>,
}

impl LifecycleJournal {
    pub fn append(&mut self, kind: LifecycleTransitionKind, metadata: String) {
        self.transitions.push(LifecycleTransition {
            sequence: self.transitions.len() as u64,
            kind,
            metadata,
        });
    }

    pub fn root(&self) -> String {
        hash_bytes(
            &bincode::serialize(&self.transitions).expect("lifecycle serialization must succeed"),
        )
    }
}
