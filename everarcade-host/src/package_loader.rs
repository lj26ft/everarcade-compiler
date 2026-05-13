use std::{fs, path::Path};

use execution_core::civilization::CivilizationPackage;

use crate::error::HostError;

pub fn load_package(path: &Path) -> Result<CivilizationPackage, HostError> {
    let bytes = fs::read(path)?;
    bincode::deserialize(&bytes).map_err(HostError::Decode)
}

pub fn save_package(path: &Path, package: &CivilizationPackage) -> Result<(), HostError> {
    let bytes = bincode::serialize(package).map_err(HostError::Encode)?;
    fs::write(path, bytes)?;
    Ok(())
}
