use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncWindow {
    pub replay_start: u64,
    pub replay_end: u64,
    pub checkpoint_start: u64,
    pub checkpoint_end: u64,
    pub settlement_start: u64,
    pub settlement_end: u64,
    pub archive_start: u64,
    pub archive_end: u64,
}

impl SyncWindow {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
