pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceQuorumRuntime {
    pub proposal_root: Hash,
    pub quorum_root: Hash,
    pub threshold: u64,
}
