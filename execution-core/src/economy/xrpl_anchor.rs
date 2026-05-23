use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct XrplSettlementAnchor {
    pub settlement_commitment: String,
    pub replay_root: String,
    pub checkpoint_root: String,
    pub ownership_root: String,
}

impl XrplSettlementAnchor {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }

    pub fn verify(
        &self,
        replay_root: &str,
        checkpoint_root: &str,
        ownership_root: &str,
    ) -> Result<(), String> {
        if self.replay_root != replay_root
            || self.checkpoint_root != checkpoint_root
            || self.ownership_root != ownership_root
        {
            return Err("xrpl anchor continuity mismatch".into());
        }
        Ok(())
    }
}
