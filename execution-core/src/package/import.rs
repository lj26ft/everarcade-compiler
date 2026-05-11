use super::{bundle::ExecutionPackage, bundle::PackageError};

pub fn import_package(bytes: &[u8], expected_epoch: u64) -> Result<ExecutionPackage, PackageError> {
    let package: ExecutionPackage =
        serde_json::from_slice(bytes).map_err(|e| PackageError::Deserialization(e.to_string()))?;
    if package.manifest.protocol_epoch != expected_epoch {
        return Err(PackageError::EpochMismatch {
            expected: expected_epoch,
            found: package.manifest.protocol_epoch,
        });
    }
    package.verify_manifest_consistency()?;
    Ok(package)
}
