use execution_core::render_bridge::RenderFrameEnvelope;

pub fn render_inventory(frame: &RenderFrameEnvelope) -> String {
    format!("inventory owner=player1 marker=item@tick{} transfer=none", frame.world.tick)
}
