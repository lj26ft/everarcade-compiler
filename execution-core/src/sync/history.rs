use serde::{Deserialize, Serialize};

use crate::operator::continuity::Hash256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorHistoryEntry {
    pub sequence: u64,
    pub checkpoint_root: Hash256,
    pub execution_id: Hash256,
}

pub fn append_cursor_history(
    history: &mut Vec<CursorHistoryEntry>,
    entry: CursorHistoryEntry,
) -> Result<(), String> {
    if let Some(last) = history.last() {
        if entry.sequence <= last.sequence {
            return Err("sequence must strictly increase".into());
        }
        if entry.sequence != last.sequence + 1 {
            return Err("sequence gap detected".into());
        }
        if entry.checkpoint_root == [0u8; 32] || entry.execution_id == [0u8; 32] {
            return Err("continuity roots must be non-zero".into());
        }
    }
    history.push(entry);
    Ok(())
}

pub fn verify_cursor_monotonicity(history: &[CursorHistoryEntry]) -> Result<(), String> {
    for pair in history.windows(2) {
        let prev = &pair[0];
        let next = &pair[1];
        if next.sequence != prev.sequence + 1 {
            return Err("sequence discontinuity".into());
        }
    }
    Ok(())
}
