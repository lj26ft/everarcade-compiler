use serde::{Deserialize, Serialize};

use super::hash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderProjectionArtifact {
    pub artifact_id: String,
    pub session_id: String,
    pub frame_index: u64,
    pub projection_root: String,
    pub projection_hash: String,
    pub parent_projection_hash: Option<String>,
    pub event_hashes: Vec<String>,
    pub timestamp: u64,
    pub frame_hash: String,
}

impl RenderProjectionArtifact {
    pub fn with_deterministic_hash(mut self) -> Result<Self, String> {
        self.frame_hash = hash::hash_serialized(&(
            &self.artifact_id,
            &self.session_id,
            self.frame_index,
            &self.projection_root,
            &self.projection_hash,
            &self.parent_projection_hash,
            &self.event_hashes,
            self.timestamp,
        ))?;
        Ok(self)
    }
}
