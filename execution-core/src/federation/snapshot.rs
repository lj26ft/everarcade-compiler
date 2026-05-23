use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationSnapshot {
    pub state_root: String,
    pub replay_root: String,
    pub checkpoint_root: String,
    pub settlement_root: String,
    pub quorum_root: String,
    pub federation_manifest_hash: String,
}

impl FederationSnapshot {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
