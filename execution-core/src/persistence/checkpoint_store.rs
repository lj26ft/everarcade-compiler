use std::{fs, path::Path};

use sha2::{Digest, Sha256};

use crate::state::{
    decode_checkpoint_with_expected_root, encode_checkpoint, CanonicalState, Hash256,
};

use super::errors::PersistenceError;

pub fn checkpoint_root(bytes: &[u8]) -> [u8; 32] {
    <[u8; 32]>::from(Sha256::digest(bytes))
}

pub fn save_checkpoint(path: &Path, state_bytes: &[u8]) -> Result<(), PersistenceError> {
    fs::write(path, state_bytes)?;
    Ok(())
}

pub fn load_checkpoint(
    path: &Path,
    expected: Option<[u8; 32]>,
) -> Result<Vec<u8>, PersistenceError> {
    let bytes = fs::read(path)?;
    if let Some(expected_root) = expected {
        let actual = checkpoint_root(&bytes);
        if actual != expected_root {
            return Err(PersistenceError::CheckpointRootMismatch {
                expected: expected_root,
                actual,
            });
        }
    }
    Ok(bytes)
}

pub fn save_state_checkpoint(
    path: &Path,
    _root: Hash256,
    state: &CanonicalState,
) -> Result<(), PersistenceError> {
    let encoded = encode_checkpoint(state).map_err(|e| {
        PersistenceError::Encode(Box::new(bincode::ErrorKind::Custom(e.to_string())))
    })?;
    fs::write(path, encoded)?;
    Ok(())
}

pub fn load_state_checkpoint(
    path: &Path,
    root: Hash256,
) -> Result<CanonicalState, PersistenceError> {
    let bytes = fs::read(path)?;
    decode_checkpoint_with_expected_root(&bytes, root)
        .map_err(|e| PersistenceError::Decode(Box::new(bincode::ErrorKind::Custom(e.to_string()))))
}
