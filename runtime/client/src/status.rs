pub fn render_status(tick: u64, entity_count: usize, world_root: &str, event_root: &str, validation_root: &str, inventory_state: &str, snapshot_count: usize, replay_count: usize) -> String {
    format!("tick={tick} entities={entity_count} world_root={world_root} event_root={event_root} validation_root={validation_root} inventory={inventory_state} snapshots={snapshot_count} replays={replay_count}")
}
