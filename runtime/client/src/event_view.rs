use execution_core::render_bridge::ProjectedEventFrame;

pub fn render_event_stream(events: &[ProjectedEventFrame]) -> String {
    events
        .iter()
        .map(|e| format!("tick={} event_root={}", e.tick, e.event_root))
        .collect::<Vec<_>>()
        .join("\n")
}
