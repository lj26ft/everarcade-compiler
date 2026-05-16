use std::{fs, path::Path};

use crate::canonical::encoding::{canonical_decode, canonical_encode};

use super::{continuity::WorldRecoveryDescriptor, errors::OperatorRecoveryError};

pub fn save_recovery_descriptor(path: &Path, descriptor: &WorldRecoveryDescriptor) -> Result<(), OperatorRecoveryError> {
    let bytes = canonical_encode(descriptor).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    }
    fs::write(path, bytes).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))
}

pub fn load_recovery_descriptor(path: &Path) -> Result<WorldRecoveryDescriptor, OperatorRecoveryError> {
    let bytes = fs::read(path).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    canonical_decode(&bytes).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))
}
