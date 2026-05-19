use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointRecord {
    pub checkpoint_id: [u8; 32],
    pub state_root: [u8; 32],
    pub journal_sequence: u64,
    pub journal_entry_hash: [u8; 32],
    pub checkpoint_hash: [u8; 32],
}

impl CheckpointRecord {
    pub fn from_parts(
        state_root: [u8; 32],
        journal_sequence: u64,
        journal_entry_hash: [u8; 32],
    ) -> Self {
        let checkpoint_id: [u8; 32] =
            Sha256::digest(bincode::serialize(&(state_root, journal_sequence)).unwrap()).into();
        let checkpoint_hash: [u8; 32] = Sha256::digest(
            bincode::serialize(&(
                checkpoint_id,
                state_root,
                journal_sequence,
                journal_entry_hash,
            ))
            .unwrap(),
        )
        .into();
        Self {
            checkpoint_id,
            state_root,
            journal_sequence,
            journal_entry_hash,
            checkpoint_hash,
        }
    }
}
