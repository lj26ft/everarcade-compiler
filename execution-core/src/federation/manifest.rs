use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationExecutionManifest {
    pub federation_epoch: u64,
    pub execution_manifest_hash: String,
    pub checkpoint_hash: String,
    pub settlement_hash: String,
    pub quorum_hash: String,
    pub participating_nodes: Vec<String>,
    pub state_root: String,
    pub replay_root: String,
}

impl FederationExecutionManifest {
    pub fn canonicalized(mut self) -> Self {
        self.participating_nodes.sort();
        self.participating_nodes.dedup();
        self
    }

    pub fn canonical_hash(&self) -> Result<String, String> {
        let canonical = self.clone().canonicalized();
        Ok(hash_bytes(
            &canonical_encode(&canonical).map_err(|e| e.to_string())?,
        ))
    }
}
