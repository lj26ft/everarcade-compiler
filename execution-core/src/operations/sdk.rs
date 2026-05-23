use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SdkProjectManifest {
    pub world_id: String,
    pub asset_package_hash: String,
    pub topology_hash: String,
    pub replay_profile: String,
    pub deployment_profile: String,
}

impl SdkProjectManifest {
    pub fn manifest_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("sdk serialization must succeed"))
    }
}
