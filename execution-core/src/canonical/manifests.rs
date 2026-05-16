use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::lineage::ExecutionLineageChain;

use super::{
    encoding::{canonical_decode, canonical_encode},
    errors::CanonicalError,
    hashes,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalExecutionManifest {
    pub package_root: [u8; 32],
    pub receipt_hash: [u8; 32],
    pub lineage_hash: [u8; 32],
    pub checkpoint_root: [u8; 32],
    pub final_state_root: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterminismVerificationReport {
    pub deterministic: bool,
    pub receipt_match: bool,
    pub lineage_match: bool,
    pub state_match: bool,
    pub manifest_match: bool,
    pub fuel_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterminismMismatch {
    pub field: String,
    pub expected: String,
    pub actual: String,
}

pub fn generate_execution_manifest(
    package_root: [u8; 32],
    receipt_hash: [u8; 32],
    lineage: &ExecutionLineageChain,
    checkpoint_root: [u8; 32],
    final_state_root: [u8; 32],
) -> CanonicalExecutionManifest {
    CanonicalExecutionManifest {
        package_root,
        receipt_hash,
        lineage_hash: hashes::lineage_hash(lineage),
        checkpoint_root,
        final_state_root,
    }
}

pub fn save_manifest(path: &Path, manifest: &CanonicalExecutionManifest) -> Result<(), CanonicalError> {
    let bytes = canonical_encode(manifest)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

pub fn load_manifest(path: &Path) -> Result<CanonicalExecutionManifest, CanonicalError> {
    canonical_decode(&fs::read(path)?)
}
