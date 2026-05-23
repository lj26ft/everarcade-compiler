use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldArchive {
    pub sequence: u64,
    pub previous_archive_hash: String,
    pub world_state_root: String,
    pub replay_root: String,
    pub checkpoint_root: String,
    pub economic_ledger_root: String,
    pub entity_lineage_root: String,
    pub federation_continuity_root: String,
}

impl WorldArchive {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}

pub fn validate_archive_lineage(chain: &[WorldArchive]) -> Result<String, String> {
    let mut previous = String::new();
    for archive in chain {
        if archive.previous_archive_hash != previous {
            return Err("archive lineage mismatch".into());
        }
        previous = archive.canonical_hash()?;
    }
    Ok(previous)
}
