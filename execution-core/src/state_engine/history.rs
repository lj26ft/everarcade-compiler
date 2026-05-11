use super::snapshot::StateSnapshot;

pub fn replay_from_snapshot(snapshot: &StateSnapshot) -> String {
    snapshot.state_root.clone()
}

pub fn verify_historical_state(snapshot: &StateSnapshot) -> bool {
    snapshot.hash() == snapshot.snapshot_hash
}
