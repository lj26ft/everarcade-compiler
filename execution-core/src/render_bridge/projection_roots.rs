use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionWindowRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionReplayRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionCheckpointRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualizationProjectionRoot(pub String);

pub fn derive_projection_root<T: Serialize>(value: &T) -> Result<ProjectionRoot, String> {
    let bytes = canonical_encode(value).map_err(|e| e.to_string())?;
    Ok(ProjectionRoot(hash_bytes(&bytes)))
}
