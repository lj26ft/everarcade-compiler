use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementEntry {
    pub execution_commitment: String,
    pub checkpoint_commitment: String,
    pub state_root_confirmation: String,
    pub replay_confirmation: String,
    pub continuity_ack: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationSettlementJournal {
    pub entries: Vec<SettlementEntry>,
}

impl FederationSettlementJournal {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
