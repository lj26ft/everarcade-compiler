#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionCostModel {
    pub fuel_units: u64,
    pub memory_bytes: u64,
    pub snapshot_bytes: u64,
    pub verifier_replay_units: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriceQuote {
    pub total_cost: u64,
}

impl ExecutionCostModel {
    pub fn quote(&self) -> PriceQuote {
        PriceQuote {
            total_cost: self.fuel_units
                .saturating_add(self.memory_bytes / 1024)
                .saturating_add(self.snapshot_bytes / 1024)
                .saturating_add(self.verifier_replay_units),
        }
    }
}
