use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CommitError {
    #[error("state changes contain duplicate key")]
    DuplicateStateKey,
    #[error("empty diff requires explicit no-op")]
    EmptyDiffRequiresNoOp,
    #[error("journal previous hash mismatch")]
    JournalPreviousHashMismatch,
    #[error("journal sequence mismatch")]
    JournalSequenceMismatch,
}
