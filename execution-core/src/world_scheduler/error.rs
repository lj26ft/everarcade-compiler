#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldSchedulerError {
    OutOfOrderTick { expected: u64, actual: u64 },
    UnknownEntity(String),
    ContinuityViolation(&'static str),
}
