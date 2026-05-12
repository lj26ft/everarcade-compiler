#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceUsage {
    pub execution_units: u64,
    pub replay_units: u64,
    pub proof_units: u64,
    pub storage_units: u64,
}

impl ResourceUsage {
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            execution_units: self.execution_units.saturating_add(other.execution_units),
            replay_units: self.replay_units.saturating_add(other.replay_units),
            proof_units: self.proof_units.saturating_add(other.proof_units),
            storage_units: self.storage_units.saturating_add(other.storage_units),
        }
    }
}
