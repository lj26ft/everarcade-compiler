use super::{error::GovernanceError, proposal::GovernanceProposal};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GovernanceVote {
    pub proposal_id: [u8; 32],
    pub voter: [u8; 32],
    pub approve: bool,
    pub epoch: u64,
    pub vote_id: [u8; 32],
}
pub fn submit_governance_vote(
    proposal: &GovernanceProposal,
    voter: [u8; 32],
    approve: bool,
    epoch: u64,
    existing: &[GovernanceVote],
) -> Result<GovernanceVote, GovernanceError> {
    if existing
        .iter()
        .any(|v| v.proposal_id == proposal.id && v.voter == voter && v.epoch == epoch)
    {
        return Err(GovernanceError::DuplicateVote);
    }
    let mut h = Sha256::new();
    h.update(proposal.id);
    h.update(voter);
    h.update([approve as u8]);
    h.update(epoch.to_le_bytes());
    Ok(GovernanceVote {
        proposal_id: proposal.id,
        voter,
        approve,
        epoch,
        vote_id: h.finalize().into(),
    })
}
pub fn verify_vote_continuity(votes: &[GovernanceVote]) -> Result<(), GovernanceError> {
    let mut seen = BTreeSet::new();
    for v in votes {
        if !seen.insert((v.proposal_id, v.voter, v.epoch)) {
            return Err(GovernanceError::DuplicateVote);
        }
    }
    Ok(())
}
