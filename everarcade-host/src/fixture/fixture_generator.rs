use std::{fs, path::Path};

use execution_core::civilization::CivilizationPackage;

use crate::{
    error::HostError,
    fixture::{
        civilization_fixture::generate_civilization_fixture_package,
        fixture_root::{package_root, Hash},
        fixture_validation::validate_fixture_package,
    },
};

pub struct FixtureGenerationResult {
    pub package_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
    pub output_path: String,
}

pub fn generate_fixture_bytes() -> Result<Vec<u8>, HostError> {
    let package = generate_civilization_fixture_package();
    if !validate_fixture_package(&package) {
        return Err(HostError::InvalidPackage);
    }
    Ok(execution_core::codec::package_encode::encode_package(
        &package,
    ))
}

pub fn generate_fixture_to_path(output: &Path) -> Result<FixtureGenerationResult, HostError> {
    let package: CivilizationPackage = generate_civilization_fixture_package();
    if !validate_fixture_package(&package) {
        return Err(HostError::InvalidPackage);
    }
    let bytes = execution_core::codec::package_encode::encode_package(&package);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, bytes)?;
    Ok(FixtureGenerationResult {
        package_root: package_root(&package),
        replay_root: package.replay_root,
        checkpoint_root: package.checkpoint_root,
        output_path: output.display().to_string(),
    })
}
