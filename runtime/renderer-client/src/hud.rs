use execution_core::render_bridge::RenderFrameEnvelope;

pub fn render_hud(frame: &RenderFrameEnvelope, entity_count: usize, inventory_count: usize, checkpoint_id: &str, replay_status: &str, session_id: &str) -> String {
    format!("hud tick={} world_root={} projection_root={} validation_root={} entity_count={} inventory_count={} checkpoint_id={} replay_status={} runtime_session_id={}", frame.world.tick, frame.world.state_root, frame.world.state_root, frame.replay.replay_root, entity_count, inventory_count, checkpoint_id, replay_status, session_id)
}
