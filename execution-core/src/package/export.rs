use crate::hashing;

use super::{bundle::ExecutionPackage, bundle::PackageError, manifest::ExecutionManifest};

pub fn build_manifest(mut package: ExecutionPackage) -> Result<ExecutionManifest, PackageError> {
    package.manifest.package_hash.clear();
    let package_hash = serde_json::to_vec(&package)
        .map(|bytes| hashing::hash_bytes(&bytes))
        .map_err(|e| PackageError::Serialization(e.to_string()))?;
    package.manifest.package_hash = package_hash;
    Ok(package.manifest)
}

pub fn export_package(package: &ExecutionPackage) -> Result<Vec<u8>, PackageError> {
    serde_json::to_vec(package).map_err(|e| PackageError::Serialization(e.to_string()))
}
