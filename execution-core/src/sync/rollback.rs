use serde::{Deserialize, Serialize};

use crate::sync::{cursor::SyncCursor, history::CursorHistoryEntry};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDetectionReport {
    pub rollback_detected: bool,
    pub expected_sequence: u64,
    pub actual_sequence: u64,
}

pub fn detect_rollback(
    stored_cursor: &SyncCursor,
    incoming_cursor: &SyncCursor,
    history: &[CursorHistoryEntry],
) -> RollbackDetectionReport {
    let mut rollback_detected = incoming_cursor.latest_sequence < stored_cursor.latest_sequence;
    if let Some(last) = history.last() {
        if incoming_cursor.latest_sequence < last.sequence {
            rollback_detected = true;
        }
        if incoming_cursor.latest_execution_id != last.execution_id
            && incoming_cursor.latest_sequence == last.sequence
        {
            rollback_detected = true;
        }
        if incoming_cursor.latest_checkpoint_root != last.checkpoint_root
            && incoming_cursor.latest_sequence == last.sequence
        {
            rollback_detected = true;
        }
    }
    RollbackDetectionReport {
        rollback_detected,
        expected_sequence: stored_cursor.latest_sequence,
        actual_sequence: incoming_cursor.latest_sequence,
    }
}
