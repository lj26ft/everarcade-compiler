use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerifierVote {
    pub verifier_id: String,
    pub receipt_hash: String,
    pub execution_root: String,
    pub snapshot_hash: String,
    pub epoch_id: u64,
}
