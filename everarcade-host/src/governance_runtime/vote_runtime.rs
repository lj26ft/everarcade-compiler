pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceVoteRuntime {
    pub proposal_root: Hash,
    pub voter_root: Hash,
    pub vote_root: Hash,
}
