use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveManifest {
    pub era: u64,
    pub previous_archive_root: String,
    pub compression_profile: String,
    pub restoration_checkpoint: String,
    pub retention_policy: String,
}

impl ArchiveManifest {
    pub fn continuity_root(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("archive serialization must succeed"))
    }
}
