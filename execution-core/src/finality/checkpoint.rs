#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalityCheckpoint {
    pub execution_root: String,
    pub receipt_root: String,
    pub snapshot_root: String,
    pub epoch_id: u64,
    pub verifier_quorum_proof: String,
}
