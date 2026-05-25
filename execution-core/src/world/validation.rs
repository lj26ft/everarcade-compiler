use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

use super::RuntimeMetrics;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeValidationRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEquivalenceRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContinuityRoot(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationArtifact {
    pub runtime_validation_root: RuntimeValidationRoot,
    pub metrics: RuntimeMetrics,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationManifest {
    pub archive_id: String,
    pub artifact_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationArchive {
    pub manifest: ValidationManifest,
    pub artifacts: Vec<ValidationArtifact>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationAnchor {
    pub validation_root: RuntimeValidationRoot,
    pub continuity_root: RuntimeContinuityRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationWindow {
    pub start_execution: u64,
    pub end_execution: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationCursor {
    pub current_window: ValidationWindow,
    pub anchor: ValidationAnchor,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationValidationSurface {
    pub export_root: RuntimeValidationRoot,
    pub archive_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationProofBundle {
    pub equivalence_root: RuntimeEquivalenceRoot,
    pub continuity_root: RuntimeContinuityRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederatedReplayAnchor {
    pub replay_root: RuntimeEquivalenceRoot,
    pub replay_window_count: u64,
}

pub fn runtime_validation_root(metrics: &RuntimeMetrics) -> Result<RuntimeValidationRoot, String> {
    Ok(RuntimeValidationRoot(hash_bytes(
        &canonical_encode(metrics).map_err(|e| e.to_string())?,
    )))
}
