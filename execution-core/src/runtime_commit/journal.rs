use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JournalEntry {
    pub sequence_number: u64,
    pub previous_entry_hash: [u8; 32],
    pub receipt_hash: [u8; 32],
    pub state_diff_hash: [u8; 32],
    pub checkpoint_hash: [u8; 32],
    pub entry_hash: [u8; 32],
}

impl JournalEntry {
    pub fn compute_hash(&self) -> [u8; 32] {
        let tuple = (
            self.sequence_number,
            self.previous_entry_hash,
            self.receipt_hash,
            self.state_diff_hash,
            self.checkpoint_hash,
        );
        Sha256::digest(bincode::serialize(&tuple).expect("serialize journal")).into()
    }
}
