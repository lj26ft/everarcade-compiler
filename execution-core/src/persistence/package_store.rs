use std::{fs, path::Path};

use sha2::{Digest, Sha256};

use super::errors::PersistenceError;

pub fn package_root(bytes: &[u8]) -> [u8; 32] { <[u8; 32]>::from(Sha256::digest(bytes)) }

pub fn save_package(path: &Path, wasm_bytes: &[u8]) -> Result<(), PersistenceError> { fs::write(path, wasm_bytes)?; Ok(()) }

pub fn load_package(path: &Path, expected: [u8; 32]) -> Result<Vec<u8>, PersistenceError> { load_package_bytes(path, Some(expected)) }

pub fn load_package_bytes(path: &Path, expected: Option<[u8; 32]>) -> Result<Vec<u8>, PersistenceError> {
    let bytes = fs::read(path)?;
    if let Some(expected_root) = expected {
        let actual = package_root(&bytes);
        if actual != expected_root { return Err(PersistenceError::PackageRootMismatch { expected: expected_root, actual }); }
    }
    Ok(bytes)
}
