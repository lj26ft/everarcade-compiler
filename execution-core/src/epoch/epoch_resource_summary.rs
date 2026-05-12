#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EpochResourceSummary {
    pub total_execution_units: u64,
    pub total_replay_units: u64,
    pub total_storage_units: u64,
    pub total_proof_units: u64,
}
