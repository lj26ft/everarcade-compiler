#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CostSchedule {
    pub execution_unit_per_step: u64,
    pub replay_unit_per_event: u64,
    pub proof_unit_per_receipt: u64,
    pub storage_unit_per_byte: u64,
}

impl Default for CostSchedule {
    fn default() -> Self {
        Self {
            execution_unit_per_step: 1,
            replay_unit_per_event: 1,
            proof_unit_per_receipt: 2,
            storage_unit_per_byte: 1,
        }
    }
}
