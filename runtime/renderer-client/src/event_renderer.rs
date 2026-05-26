use execution_core::render_bridge::RenderFrameEnvelope;

pub fn render_events(frame: &RenderFrameEnvelope) -> String {
    format!(
        "events movement@{} replay@{} save_load=none restore=none verify=ok",
        frame.event.tick, frame.replay.replay_root
    )
}
