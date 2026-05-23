use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceManifest {
    pub protocol_version: String,
    pub federation_epoch: u64,
    pub capability_root: String,
    pub topology_root: String,
    pub migration_root: String,
    pub replay_root: String,
}

impl GovernanceManifest {
    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("governance manifest serialize failed"))
    }
}
