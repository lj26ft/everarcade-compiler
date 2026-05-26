use serde::{Deserialize, Serialize};

use super::{artifact::RenderProjectionArtifact, manifest::ProjectionArtifactManifest, validation};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionArchiveManifest {
    pub archive_id: String,
    pub session_ids: Vec<String>,
    pub continuity_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionArtifactArchive {
    pub manifest: ProjectionArchiveManifest,
    pub session_manifests: Vec<ProjectionArtifactManifest>,
    pub artifacts: Vec<RenderProjectionArtifact>,
}

impl ProjectionArtifactArchive {
    pub fn verify(&self) -> Result<(), String> {
        for m in &self.session_manifests {
            validation::validate_manifest_integrity(m)?;
        }
        Ok(())
    }
}
