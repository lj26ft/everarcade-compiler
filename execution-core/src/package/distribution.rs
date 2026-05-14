use std::fs;
use std::path::Path;

use super::{
    bundle::ExecutionPackage, bundle::PackageError, export::export_package, import::import_package,
};

pub fn store_local(path: &Path, package: &ExecutionPackage) -> Result<(), PackageError> {
    let bytes = export_package(package)?;
    fs::write(path, bytes).map_err(|e| PackageError::Serialization(e.to_string()))
}

pub fn load_local(path: &Path, expected_epoch: u64) -> Result<ExecutionPackage, PackageError> {
    let bytes = fs::read(path).map_err(|e| PackageError::Deserialization(e.to_string()))?;
    import_package(&bytes, expected_epoch)
}
