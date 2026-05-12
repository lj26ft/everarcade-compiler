#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionBudget {
    pub max_execution_units: u64,
    pub max_replay_units: u64,
    pub max_proof_units: u64,
    pub max_storage_units: u64,
}
