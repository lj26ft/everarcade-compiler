use serde::{Deserialize, Serialize};

use super::errors::{Result, SyncError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncWindow {
    pub start_sequence: u64,
    pub end_sequence: u64,
}

pub fn validate_sync_window(window: &SyncWindow) -> Result<()> {
    if window.start_sequence > window.end_sequence {
        return Err(SyncError::mismatch(
            "sync_window",
            "ordered(start<=end)",
            format!("{}>{}", window.start_sequence, window.end_sequence),
        ));
    }
    Ok(())
}
