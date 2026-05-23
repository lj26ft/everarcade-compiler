use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCompressionManifest {
    pub range_start: u64,
    pub range_end: u64,
    pub snapshot_root: String,
    pub continuity_anchor_root: String,
    pub compressed_chunks: u32,
}

impl ReplayCompressionManifest {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }

    pub fn restores_to(&self, expected_snapshot_root: &str, expected_anchor_root: &str) -> bool {
        self.snapshot_root == expected_snapshot_root
            && self.continuity_anchor_root == expected_anchor_root
    }
}
