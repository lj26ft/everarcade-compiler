#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeLoadProfile {
    pub stage_count: usize,
    pub replay_window_count: usize,
    pub partition_count: usize,
}

impl RuntimeLoadProfile {
    pub fn deterministic_capacity_score(&self) -> usize {
        self.stage_count
            .saturating_mul(self.replay_window_count)
            .saturating_add(self.partition_count)
    }
}
