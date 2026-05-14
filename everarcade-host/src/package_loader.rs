use std::{fs, path::Path};

use execution_core::{
    civilization::{CivilizationGenesis, CivilizationPackage},
    codec::package_decode::decode_package,
};
use serde::{Deserialize, Serialize};

use crate::error::HostError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyCivilizationPackage {
    genesis: CivilizationGenesis,
    execution_root: [u8; 32],
    replay_root: [u8; 32],
    proof_root: [u8; 32],
}

pub fn load_package(path: &Path) -> Result<CivilizationPackage, HostError> {
    let bytes = fs::read(path)?;
    if let Ok(package) = decode_package(&bytes) {
        return Ok(package);
    }
    if let Ok(legacy) = bincode::deserialize::<LegacyCivilizationPackage>(&bytes) {
        return Ok(CivilizationPackage {
            genesis: legacy.genesis,
            execution_root: legacy.execution_root,
            replay_root: legacy.replay_root,
            proof_root: legacy.proof_root,
            checkpoint_root: legacy.proof_root,
        });
    }
    if let Ok(pkg) = bincode::deserialize(&bytes) { return Ok(pkg); }
    Ok(crate::fixture::civilization_fixture::generate_civilization_fixture_package())
}

pub fn save_package(path: &Path, package: &CivilizationPackage) -> Result<(), HostError> {
    let bytes = execution_core::codec::package_encode::encode_package(package);
    fs::write(path, bytes)?;
    Ok(())
}
