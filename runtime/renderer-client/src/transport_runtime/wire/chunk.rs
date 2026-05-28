use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayChunkWireMessage {
    pub stream_id: String,
    pub sequence: u64,
    pub previous_hash: String,
    pub continuity_root: String,
    pub payload: Vec<u8>,
    pub payload_hash: String,
    pub reconstruction_only: bool,
}

pub fn deterministic_hash(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

impl ReplayChunkWireMessage {
    pub fn new(
        stream_id: impl Into<String>,
        sequence: u64,
        previous_hash: impl Into<String>,
        continuity_root: impl Into<String>,
        payload: Vec<u8>,
    ) -> Self {
        let payload_hash = deterministic_hash(&payload);
        Self {
            stream_id: stream_id.into(),
            sequence,
            previous_hash: previous_hash.into(),
            continuity_root: continuity_root.into(),
            payload,
            payload_hash,
            reconstruction_only: true,
        }
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(self).map_err(|e| format!("wire_serialize_error:{e}"))
    }

    pub fn wire_hash(&self) -> Result<String, String> {
        self.canonical_bytes().map(|b| deterministic_hash(&b))
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.reconstruction_only {
            return Err("authority_mutation_rejected".into());
        }
        if self.stream_id.is_empty() {
            return Err("malformed_replay_chunk".into());
        }
        if self.payload_hash != deterministic_hash(&self.payload) {
            return Err("replay_corruption_rejected".into());
        }
        if self.sequence > 0 && self.previous_hash.is_empty() {
            return Err("replay_injection_rejected".into());
        }
        if self.continuity_root.is_empty() {
            return Err("invalid_continuity_root".into());
        }
        Ok(())
    }
}
