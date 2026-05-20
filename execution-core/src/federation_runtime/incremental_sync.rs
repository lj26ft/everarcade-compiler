use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncrementalSyncState {
    pub last_journal: u64,
    pub last_receipt: u64,
    pub last_checkpoint: u64,
    pub last_replay_proof: u64,
}

pub fn validate_incremental_advancement(previous: u64, next: u64) -> bool {
    next == previous.saturating_add(1)
}

pub fn advance_incremental_sync(
    state: &mut IncrementalSyncState,
    next_journal: u64,
    next_receipt: u64,
    next_checkpoint: u64,
    next_replay_proof: u64,
) -> bool {
    let ordered = validate_incremental_advancement(state.last_journal, next_journal)
        && validate_incremental_advancement(state.last_receipt, next_receipt)
        && validate_incremental_advancement(state.last_checkpoint, next_checkpoint)
        && validate_incremental_advancement(state.last_replay_proof, next_replay_proof);

    if !ordered {
        return false;
    }

    state.last_journal = next_journal;
    state.last_receipt = next_receipt;
    state.last_checkpoint = next_checkpoint;
    state.last_replay_proof = next_replay_proof;
    true
}

pub fn verify_incremental_receipts(last_receipt: u64, next_receipt: u64) -> bool {
    validate_incremental_advancement(last_receipt, next_receipt)
}

pub fn advance_checkpoint_continuity(
    state: &mut IncrementalSyncState,
    next_checkpoint: u64,
) -> bool {
    if !validate_incremental_advancement(state.last_checkpoint, next_checkpoint) {
        return false;
    }
    state.last_checkpoint = next_checkpoint;
    true
}
