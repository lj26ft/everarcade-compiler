use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::wasm::receipt::Hash256;

use super::{chain::ExecutionLineageChain, errors::LineageError};

pub fn lineage_path_for_world(root: &Path, world_id: Hash256) -> PathBuf {
    root.join("data")
        .join("worlds")
        .join(hex::encode(world_id))
        .join("lineage.bin")
}

pub fn save_lineage(path: &Path, chain: &ExecutionLineageChain) -> Result<(), LineageError> {
    let bytes = bincode::serialize(chain).map_err(LineageError::Encode)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

pub fn load_lineage(path: &Path) -> Result<ExecutionLineageChain, LineageError> {
    let bytes = fs::read(path)?;
    bincode::deserialize(&bytes).map_err(LineageError::Decode)
}
