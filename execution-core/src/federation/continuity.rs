use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityGuarantee {
    pub continuity_hash: String,
}

impl ContinuityGuarantee {
    pub fn from_roots(previous: &str, next: &str) -> Self {
        Self {
            continuity_hash: hash_bytes(format!("{previous}->{next}").as_bytes()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationContinuityProof {
    pub previous_root: String,
    pub current_root: String,
    pub checkpoint_hash: String,
    pub continuity_hash: String,
}

impl FederationContinuityProof {
    pub fn new(previous_root: String, current_root: String, checkpoint_hash: String) -> Self {
        let continuity_hash =
            hash_bytes(format!("{previous_root}|{current_root}|{checkpoint_hash}").as_bytes());
        Self {
            previous_root,
            current_root,
            checkpoint_hash,
            continuity_hash,
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        let expected = hash_bytes(
            format!(
                "{}|{}|{}",
                self.previous_root, self.current_root, self.checkpoint_hash
            )
            .as_bytes(),
        );
        if self.continuity_hash != expected {
            return Err("continuity hash mismatch".into());
        }
        Ok(())
    }

    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
