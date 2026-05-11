use everarcade_abi::StateChange;

use super::{merkle, snapshot::StateSnapshot, store::StateStore};

pub fn apply_state_changes(
    store: &mut StateStore,
    changes: &[StateChange],
    previous_snapshot_hash: Option<String>,
) -> (String, StateSnapshot) {
    store.apply_changes(changes);
    let root = merkle::to_hex(&store.root());
    let snapshot = store.snapshot(previous_snapshot_hash);
    (root, snapshot)
}
