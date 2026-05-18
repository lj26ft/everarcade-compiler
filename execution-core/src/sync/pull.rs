use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{
    cursor::SyncCursor,
    errors::{Result, SyncError},
    window::{validate_sync_window, SyncWindow},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullRequest {
    pub world_id: String,
    pub from_cursor: SyncCursor,
    pub requested_window: SyncWindow,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PullResponse {
    pub checkpoint: Option<PathBuf>,
    pub receipts: Vec<PathBuf>,
    pub lineage: PathBuf,
    pub manifest: PathBuf,
}

pub fn validate_pull_response(
    response: &PullResponse,
    requested_window: &SyncWindow,
) -> Result<()> {
    validate_sync_window(requested_window)?;
    let expected = (requested_window.end_sequence - requested_window.start_sequence + 1) as usize;
    if response.receipts.len() != expected {
        return Err(SyncError::mismatch(
            "receipts",
            expected.to_string(),
            response.receipts.len().to_string(),
        ));
    }
    Ok(())
}
