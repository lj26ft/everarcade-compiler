use std::{fs, path::Path};

use crate::{
    canonical::encoding::{canonical_decode, canonical_encode},
    sync::observer::{hash_observer_state, ObserverState},
};

pub fn save_observer_state(world_root: &Path, state: &ObserverState) -> Result<(), String> {
    let sync_dir = world_root.join("sync");
    fs::create_dir_all(&sync_dir).map_err(|e| e.to_string())?;
    let mut bytes = canonical_encode(state).map_err(|e| e.to_string())?;
    let hash = hash_observer_state(state);
    bytes.extend_from_slice(&hash);
    fs::write(sync_dir.join("observer_state.bin"), bytes).map_err(|e| e.to_string())
}

pub fn load_observer_state(world_root: &Path) -> Result<ObserverState, String> {
    let path = world_root.join("sync").join("observer_state.bin");
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    if bytes.len() < 32 {
        return Err("observer_state.bin too short".into());
    }
    let (payload, expected_hash_bytes) = bytes.split_at(bytes.len() - 32);
    let state: ObserverState = canonical_decode(payload).map_err(|e| e.to_string())?;
    let actual_hash = hash_observer_state(&state);
    if actual_hash.as_slice() != expected_hash_bytes {
        return Err("observer state root verification failed".into());
    }
    Ok(state)
}
