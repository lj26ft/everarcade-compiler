use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationEpochManifest {
    pub epoch_id: u64,
    pub federation_manifest_hash: String,
    pub checkpoint_hash: String,
    pub topology_hash: String,
    pub capability_hash: String,
    pub replay_root: String,
    pub continuity_root: String,
}

impl FederationEpochManifest {
    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("epoch manifest serialize failed"))
    }
}
