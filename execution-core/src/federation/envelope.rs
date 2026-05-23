use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationContinuityEnvelope {
    pub version: u16,
    pub execution_manifest_hash: String,
    pub checkpoint_hash: String,
    pub settlement_root: String,
    pub replay_root: String,
    pub node_identity: String,
    pub continuity_proof_hash: String,
    pub detached_signature: Option<String>,
}

impl FederationContinuityEnvelope {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
