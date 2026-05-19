use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::error::CommitError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateChange {
    pub key: Vec<u8>,
    pub before: Vec<u8>,
    pub after: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateDiff {
    pub changes: Vec<StateChange>,
    pub previous_state_root: [u8; 32],
    pub new_state_root: [u8; 32],
}

pub fn compute_state_root(previous_state_root: [u8; 32], changes: &[StateChange]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(previous_state_root);
    for c in changes {
        h.update((c.key.len() as u64).to_le_bytes());
        h.update(&c.key);
        h.update((c.before.len() as u64).to_le_bytes());
        h.update(&c.before);
        h.update((c.after.len() as u64).to_le_bytes());
        h.update(&c.after);
    }
    h.finalize().into()
}

pub fn canonicalize_changes(
    mut changes: Vec<StateChange>,
    is_noop: bool,
) -> Result<Vec<StateChange>, CommitError> {
    changes.sort_by(|a, b| a.key.cmp(&b.key));
    if changes.is_empty() && !is_noop {
        return Err(CommitError::EmptyDiffRequiresNoOp);
    }
    for w in changes.windows(2) {
        if w[0].key == w[1].key {
            return Err(CommitError::DuplicateStateKey);
        }
    }
    Ok(changes)
}

pub fn state_diff_hash(diff: &StateDiff) -> [u8; 32] {
    Sha256::digest(bincode::serialize(diff).expect("serialize diff")).into()
}
