#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTransition {
    pub state_before: u64,
    pub operation: u64,
    pub memory_transition: u64,
    pub fuel_transition: u64,
    pub state_after: u64,
}

impl RuntimeTransition {
    pub fn apply(state_before: u64, operation: u64, memory_transition: u64, fuel_transition: u64) -> Self {
        let state_after = state_before ^ operation ^ memory_transition ^ fuel_transition;
        Self { state_before, operation, memory_transition, fuel_transition, state_after }
    }
}
