use crate::{hashing::sha256, receipt_canonical::CanonicalExecutionReceipt};
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub sequence: u64,
    pub input_hash: Hash256,
    pub receipt_hash: Hash256,
    pub state_root_after: Hash256,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionJournal {
    pub entries: Vec<JournalEntry>,
}

impl ExecutionJournal {
    pub fn append(&mut self, input_hash: Hash256, receipt: &CanonicalExecutionReceipt) {
        let sequence = self.entries.len() as u64;
        self.entries.push(JournalEntry {
            sequence,
            input_hash,
            receipt_hash: receipt.canonical_hash(),
            state_root_after: receipt.state_root_after,
        });
    }

    pub fn canonical_hash(&self) -> Hash256 {
        let bytes = bincode::serialize(self).expect("journal serialization failed");
        sha256(&bytes)
    }
}
