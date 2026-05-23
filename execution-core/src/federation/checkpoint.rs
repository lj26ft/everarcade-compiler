use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationCheckpoint {
    pub federation_epoch: u64,
    pub previous_checkpoint_hash: String,
    pub state_root: String,
    pub execution_journal_hash: String,
    pub receipt_root: String,
    pub replay_root: String,
    pub settlement_root: String,
}

impl FederationCheckpoint {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}

pub fn validate_checkpoint_linkage(chain: &[FederationCheckpoint]) -> Result<(), String> {
    let mut previous = String::new();
    for checkpoint in chain {
        if checkpoint.previous_checkpoint_hash != previous {
            return Err("checkpoint linkage mismatch".into());
        }
        previous = checkpoint.canonical_hash()?;
    }
    Ok(())
}
