use super::chunk::{deterministic_hash, ReplayChunkWireMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayWindowWireMessage {
    pub window_id: String,
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub continuity_root: String,
    pub chunks: Vec<ReplayChunkWireMessage>,
    pub reconstruction_only: bool,
}

impl ReplayWindowWireMessage {
    pub fn validate(&self) -> Result<(), String> {
        if !self.reconstruction_only {
            return Err("observer_authority_mutation_rejected".into());
        }
        let mut expected = self.start_sequence;
        for chunk in &self.chunks {
            chunk.validate()?;
            if chunk.sequence != expected {
                return Err("window_order_rejected".into());
            }
            expected += 1;
        }
        if !self.chunks.is_empty() && expected - 1 != self.end_sequence {
            return Err("window_bounds_rejected".into());
        }
        Ok(())
    }
    pub fn wire_hash(&self) -> Result<String, String> {
        serde_json::to_vec(self)
            .map(|b| deterministic_hash(&b))
            .map_err(|e| e.to_string())
    }
}
