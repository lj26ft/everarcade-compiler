use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("out of order tick: expected {expected}, got {got}")]
    OutOfOrderTick { expected: u64, got: u64 },
    #[error("simulation hash mismatch")]
    HashMismatch,
    #[error("interaction verification failed")]
    InteractionVerificationFailed,
}
