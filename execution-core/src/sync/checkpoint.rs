use serde::{Deserialize, Serialize};

use crate::operator::continuity::Hash256;

use super::errors::{Result, SyncError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointAdvertisement {
    pub checkpoint_root: Hash256,
    pub sequence: u64,
}

pub fn validate_checkpoint_advertisement(a: &CheckpointAdvertisement) -> Result<()> {
    if a.checkpoint_root == [0u8; 32] {
        return Err(SyncError::mismatch("checkpoint_root", "non-zero", "zero"));
    }
    Ok(())
}
