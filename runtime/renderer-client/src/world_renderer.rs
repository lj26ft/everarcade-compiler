use execution_core::render_bridge::RenderFrameEnvelope;

pub fn render_world(frame: &RenderFrameEnvelope) -> String {
    format!(
        "world[tick={}] player=@({},{}) bounds=[0..15,0..7]",
        frame.world.tick,
        frame.world.tick % 16,
        frame.world.tick % 8
    )
}
