#[derive(Debug, Clone, Copy)]
pub struct ExecutionLimits {
    pub fuel: u64,
    pub max_memory: usize,
    pub max_stack: usize,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self { fuel: 10_000_000, max_memory: 1024 * 1024, max_stack: 1024 * 1024 }
    }
}
