#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityCheckpoint {
    pub continuity_root: String,
    pub replay_checkpoint: String,
    pub proof_checkpoint: String,
    pub settlement_checkpoint: String,
}
