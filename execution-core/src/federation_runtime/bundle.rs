use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::error::FederationRuntimeError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiptBundle {
    pub receipt_hashes: Vec<[u8; 32]>,
    pub execution_hashes: Vec<[u8; 32]>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JournalBundle {
    pub journal_hash: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointBundle {
    pub checkpoint_hash: [u8; 32],
    pub checkpoint_root: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayProofBundle {
    pub state_root: [u8; 32],
    pub continuity_hash: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContinuityBundle {
    pub state_root: [u8; 32],
    pub checkpoint_hash: [u8; 32],
    pub journal_hash: [u8; 32],
    pub receipt_hashes: Vec<[u8; 32]>,
    pub execution_hashes: Vec<[u8; 32]>,
    pub continuity_hash: [u8; 32],
}

impl ContinuityBundle {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FederationRuntimeError> {
        serde_json::to_vec(self).map_err(|e| FederationRuntimeError::Serialization(e.to_string()))
    }

    pub fn deterministic_hash(&self) -> Result<[u8; 32], FederationRuntimeError> {
        let bytes = self.canonical_bytes()?;
        Ok(Sha256::digest(bytes).into())
    }
}
