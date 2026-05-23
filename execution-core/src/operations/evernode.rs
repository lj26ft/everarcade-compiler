use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvernodeDeploymentManifest {
    pub deployment_manifest: String,
    pub anchor_coordination: String,
    pub settlement_reference: String,
    pub appliance_continuity: String,
    pub topology_anchor: String,
}

impl EvernodeDeploymentManifest {
    pub fn integration_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("evernode serialization must succeed"))
    }
}
