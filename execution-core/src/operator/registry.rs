use std::{fs, path::Path};

use crate::canonical::encoding::{canonical_decode, canonical_encode};

use super::{
    continuity::{descriptor_hash, StoredRecoveryDescriptor, WorldRecoveryDescriptor},
    errors::OperatorRecoveryError,
};

pub fn save_recovery_descriptor(
    path: &Path,
    descriptor: &WorldRecoveryDescriptor,
) -> Result<(), OperatorRecoveryError> {
    let stored = StoredRecoveryDescriptor {
        descriptor: descriptor.clone(),
        descriptor_hash: descriptor_hash(descriptor),
    };
    let bytes =
        canonical_encode(&stored).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    }
    fs::write(path, bytes).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))
}

pub fn load_recovery_descriptor(
    path: &Path,
) -> Result<WorldRecoveryDescriptor, OperatorRecoveryError> {
    let bytes = fs::read(path).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    let stored: StoredRecoveryDescriptor =
        canonical_decode(&bytes).map_err(|e| OperatorRecoveryError::Storage(e.to_string()))?;
    let computed = descriptor_hash(&stored.descriptor);
    if computed != stored.descriptor_hash {
        return Err(OperatorRecoveryError::Validation(
            super::continuity::OperatorRecoveryMismatch {
                field: "descriptor_hash".into(),
                expected: hex::encode(stored.descriptor_hash),
                actual: hex::encode(computed),
            },
        ));
    }
    Ok(stored.descriptor)
}
