#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderTick {
    pub sequence: u64,
    pub frame_tick: u64,
}

pub fn build_render_ticks(frame_ticks: impl IntoIterator<Item = u64>) -> Vec<RenderTick> {
    frame_ticks
        .into_iter()
        .enumerate()
        .map(|(i, t)| RenderTick {
            sequence: i as u64,
            frame_tick: t,
        })
        .collect()
}
