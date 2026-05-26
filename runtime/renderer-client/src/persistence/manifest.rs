use serde::{Deserialize, Serialize};

use super::hash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionArtifactManifest {
    pub manifest_id: String,
    pub session_id: String,
    pub artifact_hashes: Vec<String>,
    pub continuity_root: String,
    pub artifact_count: u64,
}

impl ProjectionArtifactManifest {
    pub fn deterministic_root(&self) -> Result<String, String> {
        hash::hash_serialized(self)
    }
}
