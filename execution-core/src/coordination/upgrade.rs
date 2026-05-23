use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolUpgradeManifest {
    pub protocol_version: String,
    pub previous_protocol_version: String,
    pub upgrade_epoch: u64,
    pub compatibility_hash: String,
    pub migration_hash: String,
    pub replay_root_before: String,
    pub replay_root_after: String,
    pub continuity_hash: String,
}

impl ProtocolUpgradeManifest {
    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("upgrade manifest serialize failed"))
    }

    pub fn validate_lineage(&self, expected_previous: &str) -> bool {
        self.previous_protocol_version == expected_previous
    }
}
