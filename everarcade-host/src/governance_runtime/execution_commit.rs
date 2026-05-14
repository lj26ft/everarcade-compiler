pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceExecutionCommit {
    pub proposal_root: Hash,
    pub quorum_root: Hash,
    pub execution_window_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
}
