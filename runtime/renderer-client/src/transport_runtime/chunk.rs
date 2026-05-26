use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayChunk {
    pub stream_id: String,
    pub sequence: u64,
    pub payload: Vec<u8>,
    pub continuity: ReplayChunkContinuity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayChunkManifest {
    pub stream_id: String,
    pub sequence: u64,
    pub payload_hash: String,
    pub continuity_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayChunkContinuity {
    pub previous_hash: String,
    pub continuity_hash: String,
}

pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

impl ReplayChunk {
    pub fn manifest(&self) -> ReplayChunkManifest {
        ReplayChunkManifest {
            stream_id: self.stream_id.clone(),
            sequence: self.sequence,
            payload_hash: hash_bytes(&self.payload),
            continuity_hash: self.continuity.continuity_hash.clone(),
        }
    }
}
