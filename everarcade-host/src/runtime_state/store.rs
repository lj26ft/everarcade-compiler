use std::{fs, path::Path};

use execution_core::runtime_commit::StateDiff;
use sha2::{Digest, Sha256};

use super::{error::RuntimeStateError, serialization::RuntimeStateMap};

pub type RuntimeStateStore = RuntimeStateMap;

pub fn load_state(world_root: &Path) -> Result<RuntimeStateStore, RuntimeStateError> {
    let state_file = world_root.join("state").join("latest_state.json");
    if !state_file.exists() {
        return Ok(RuntimeStateStore::new());
    }
    let bytes = fs::read(state_file)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn apply_state_diff(state: &mut RuntimeStateStore, diff: &StateDiff) {
    for c in &diff.changes {
        state.insert(c.key.clone(), c.after.clone());
    }
}

pub fn persist_state(
    world_root: &Path,
    state: &RuntimeStateStore,
) -> Result<[u8; 32], RuntimeStateError> {
    fs::create_dir_all(world_root.join("state"))?;
    let bytes = serde_json::to_vec(state)?;
    let root: [u8; 32] = Sha256::digest(&bytes).into();
    let latest = world_root.join("state").join("latest_state.json");
    let tmp = latest.with_extension("tmp");
    fs::write(&tmp, &bytes)?;
    fs::rename(tmp, latest)?;
    Ok(root)
}
