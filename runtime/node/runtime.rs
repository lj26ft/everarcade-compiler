use super::state::DeterministicServiceState;

#[derive(Debug, Clone)]
pub struct SovereignRuntimeNodeRuntime {
    pub state: DeterministicServiceState,
}

impl SovereignRuntimeNodeRuntime {
    pub fn start_deterministically(mut self) -> Self { self.state.operational = true; self }
    pub fn shutdown_safely(mut self) -> Self { self.state.operational = false; self }
}
