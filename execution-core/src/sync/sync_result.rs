use crate::merkle::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyncFailure {
    RequestOutOfRange,
    InvalidProofExchange,
    InvalidReceiptRange,
    InvalidCheckpoint,
    ReplayRootMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncResult {
    pub converged: bool,
    pub final_state_root: Hash,
    pub final_replay_root: Hash,
    pub final_receipt_root: Hash,
    pub failure: Option<SyncFailure>,
}
