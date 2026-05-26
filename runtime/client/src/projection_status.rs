#[allow(dead_code)]
pub fn render_projection_status(
    projection_tick: u64,
    projection_root: &str,
    frame_root: &str,
    checkpoint_root: &str,
    projection_replay_root: &str,
    window_count: usize,
    frame_count: usize,
) -> String {
    format!("projection tick={projection_tick} projection root={projection_root} frame root={frame_root} checkpoint root={checkpoint_root} projection replay root={projection_replay_root} window count={window_count} frame count={frame_count}")
}
