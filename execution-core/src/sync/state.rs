use serde::{Deserialize, Serialize};

use super::{
    cursor::SyncCursor,
    errors::{Result, SyncError},
    window::SyncWindow,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynchronizationState {
    pub current_cursor: SyncCursor,
    pub last_verified_sequence: u64,
    pub synchronized: bool,
}

pub fn advance_sync_state(
    state: &mut SynchronizationState,
    next_cursor: SyncCursor,
    window: &SyncWindow,
) -> Result<()> {
    if window.start_sequence != state.last_verified_sequence + 1 {
        return Err(SyncError::mismatch(
            "start_sequence",
            (state.last_verified_sequence + 1).to_string(),
            window.start_sequence.to_string(),
        ));
    }
    if next_cursor.latest_sequence < state.current_cursor.latest_sequence {
        return Err(SyncError::mismatch(
            "latest_sequence",
            format!(">={}", state.current_cursor.latest_sequence),
            next_cursor.latest_sequence.to_string(),
        ));
    }
    state.current_cursor = next_cursor;
    state.last_verified_sequence = window.end_sequence;
    state.synchronized = state.current_cursor.latest_sequence == state.last_verified_sequence;
    Ok(())
}
