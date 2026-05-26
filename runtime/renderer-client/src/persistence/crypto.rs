use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionHash(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionContinuityHash(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionArchiveHash(pub String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionManifestHash(pub String);

fn canonical_json<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("serialization failure: {e}"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn hash_projection<T: Serialize>(value: &T) -> Result<ProjectionHash, String> {
    let json = canonical_json(value)?;
    Ok(ProjectionHash(sha256_hex(json.as_bytes())))
}

pub fn hash_manifest<T: Serialize>(value: &T) -> Result<ProjectionManifestHash, String> {
    hash_projection(value).map(|h| ProjectionManifestHash(h.0))
}

pub fn hash_archive<T: Serialize>(value: &T) -> Result<ProjectionArchiveHash, String> {
    hash_projection(value).map(|h| ProjectionArchiveHash(h.0))
}

pub fn hash_continuity(previous: Option<&str>, payload_hash: &str) -> ProjectionContinuityHash {
    let prev = previous.unwrap_or("genesis");
    ProjectionContinuityHash(sha256_hex(format!("{prev}:{payload_hash}").as_bytes()))
}
