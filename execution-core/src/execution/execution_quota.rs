#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionQuota {
    pub max_units_per_epoch: u64,
}

impl ExecutionQuota {
    pub fn allows(&self, consumed: u64) -> bool {
        consumed <= self.max_units_per_epoch
    }
}
