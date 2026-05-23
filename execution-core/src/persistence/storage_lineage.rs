use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLineage {
    pub archive_lineage_root: String,
    pub checkpoint_lineage_root: String,
    pub replay_lineage_root: String,
    pub economic_lineage_root: String,
    pub migration_lineage_root: String,
}

impl StorageLineage {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
