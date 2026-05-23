use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SovereignPersistenceManifest {
    pub sovereign_identity: String,
    pub continuity_root: String,
    pub migration_lineage_root: String,
    pub checkpoint_lineage_root: String,
    pub replay_restoration_root: String,
}

impl SovereignPersistenceManifest {
    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("persistence serialize failed"))
    }

    pub fn verify_restoration(&self, recovered: &SovereignPersistenceManifest) -> bool {
        self.sovereign_identity == recovered.sovereign_identity
            && self.continuity_root == recovered.continuity_root
            && self.replay_restoration_root == recovered.replay_restoration_root
    }
}
