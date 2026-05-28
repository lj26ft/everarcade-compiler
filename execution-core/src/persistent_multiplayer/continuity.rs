use super::world_sync::WorldSyncState;
pub fn continuity_root(states: &[WorldSyncState]) -> String {
    let tips = states
        .iter()
        .map(|s| s.sync_root.as_str())
        .collect::<Vec<_>>()
        .join("|");
    format!("persistent-multiplayer:continuity:{tips}")
}
