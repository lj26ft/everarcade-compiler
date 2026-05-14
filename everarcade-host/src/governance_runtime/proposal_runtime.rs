pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceProposalRuntime {
    pub proposal_root: Hash,
    pub proposer_root: Hash,
    pub scope_root: Hash,
}
