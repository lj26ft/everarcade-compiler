use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceCheckpoint {
    pub epoch: u64,
    pub proposal_root: [u8; 32],
    pub vote_root: [u8; 32],
    pub policy_root: [u8; 32],
    pub authority_root: [u8; 32],
}
