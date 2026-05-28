use super::chunk::deterministic_hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayCheckpointWireMessage {
    pub checkpoint_sequence: u64,
    pub replay_tip: u64,
    pub continuity_root: String,
    pub checkpoint_hash: String,
    pub reconstruction_only: bool,
}

impl ReplayCheckpointWireMessage {
    pub fn validate(&self) -> Result<(), String> {
        if !self.reconstruction_only {
            return Err("authority_checkpoint_rejected".into());
        }
        let expected =
            deterministic_hash(format!("{}:{}", self.replay_tip, self.continuity_root).as_bytes());
        if self.checkpoint_hash != expected {
            return Err("checkpoint_corruption_rejected".into());
        }
        Ok(())
    }
}
