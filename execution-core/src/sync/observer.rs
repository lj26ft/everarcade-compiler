use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256, sync::cursor::SyncCursor,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObserverState {
    pub world_id: String,
    pub operator: FederationNodeId,
    pub current_cursor: SyncCursor,
    pub highest_verified_sequence: u64,
    pub latest_checkpoint_root: Hash256,
    pub synchronized: bool,
}

pub fn hash_observer_state(state: &ObserverState) -> Hash256 {
    Sha256::digest(&canonical_encode(state).expect("observer state encode")).into()
}

pub fn verify_observer_state(state: &ObserverState) -> Result<(), String> {
    if state.world_id.is_empty() {
        return Err("world_id empty".into());
    }
    if state.current_cursor.latest_sequence != state.highest_verified_sequence {
        return Err("cursor/latest verified sequence mismatch".into());
    }
    if state.current_cursor.latest_checkpoint_root != state.latest_checkpoint_root {
        return Err("checkpoint root mismatch".into());
    }
    Ok(())
}

pub fn advance_observer(
    state: &mut ObserverState,
    next_cursor: SyncCursor,
    synchronized: bool,
) -> Result<(), String> {
    if next_cursor.latest_sequence <= state.highest_verified_sequence {
        return Err("non-monotonic sequence".into());
    }
    if next_cursor.latest_sequence != state.highest_verified_sequence + 1 {
        return Err("skipped sequence range".into());
    }
    if next_cursor.latest_lineage_hash == [0u8; 32] {
        return Err("lineage continuity missing".into());
    }
    state.highest_verified_sequence = next_cursor.latest_sequence;
    state.latest_checkpoint_root = next_cursor.latest_checkpoint_root;
    state.current_cursor = next_cursor;
    state.synchronized = synchronized;
    Ok(())
}
