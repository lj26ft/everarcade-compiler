use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryManifest {
    pub checkpoint_restoration: String,
    pub replay_restoration: String,
    pub topology_restoration: String,
    pub settlement_restoration: String,
    pub archive_restoration: String,
}

impl RecoveryManifest {
    pub fn recovery_root(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("recovery serialization must succeed"))
    }
}
