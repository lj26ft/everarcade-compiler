#[derive(Debug, Clone, Default)]
pub struct ReplayBackpressureWindow {
    pub budget: ReplayWindowBudget,
    pub throttle: ReplayWindowThrottle,
}

#[derive(Debug, Clone, Default)]
pub struct ReplayWindowBudget {
    pub max_chunks_in_window: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ReplayWindowThrottle {
    pub inflight_chunks: usize,
}

impl ReplayBackpressureWindow {
    pub fn can_accept(&self) -> bool {
        self.throttle.inflight_chunks < self.budget.max_chunks_in_window
    }
}
