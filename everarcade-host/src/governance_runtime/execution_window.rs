pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceExecutionWindow {
    pub window_root: Hash,
    pub proposal_root: Hash,
    pub quorum_root: Hash,
    pub execution_scope_root: Hash,
}
