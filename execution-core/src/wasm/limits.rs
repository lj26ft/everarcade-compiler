#[derive(Debug, Clone, Copy)]
pub struct ExecutionLimits {
    pub fuel: u64,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self { fuel: 10_000_000 }
    }
}
