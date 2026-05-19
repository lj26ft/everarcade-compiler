use super::{error::CommitError, CommitOutput, JournalEntry};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReport {
    pub verified_entries: usize,
    pub reconstructed_state_root: [u8; 32],
}

pub fn replay_verify(entries: &[CommitOutput]) -> Result<ReplayReport, CommitError> {
    let mut prev = [0u8; 32];
    let mut seq = 0u64;
    let mut root = [0u8; 32];
    for c in entries {
        let j: &JournalEntry = &c.journal_entry;
        if j.sequence_number != seq {
            return Err(CommitError::JournalSequenceMismatch);
        }
        if j.previous_entry_hash != prev {
            return Err(CommitError::JournalPreviousHashMismatch);
        }
        prev = j.entry_hash;
        seq += 1;
        root = c.state_diff.new_state_root;
    }
    Ok(ReplayReport {
        verified_entries: entries.len(),
        reconstructed_state_root: root,
    })
}
