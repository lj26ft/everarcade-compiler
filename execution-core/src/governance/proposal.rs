use super::error::GovernanceError;
use crate::codec::canonical::canonical_bytes;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceProposal {
    pub id: [u8; 32],
    pub epoch: u64,
    pub payload: String,
    pub parent: Option<[u8; 32]>,
}
pub fn create_governance_proposal(
    epoch: u64,
    payload: String,
    parent: Option<[u8; 32]>,
) -> GovernanceProposal {
    let mut h = Sha256::new();
    h.update(epoch.to_le_bytes());
    h.update(payload.as_bytes());
    if let Some(p) = parent {
        h.update(p);
    }
    GovernanceProposal {
        id: h.finalize().into(),
        epoch,
        payload,
        parent,
    }
}
pub fn verify_proposal_lineage(proposals: &[GovernanceProposal]) -> Result<(), GovernanceError> {
    let mut ordered = proposals.to_vec();
    ordered.sort_by_key(|p| (p.epoch, p.id));
    for w in ordered.windows(2) {
        if w[1].parent != Some(w[0].id) {
            return Err(GovernanceError::ProposalLineage);
        }
        if canonical_bytes(&w[0]) == canonical_bytes(&w[1]) {
            return Err(GovernanceError::ProposalLineage);
        }
    }
    Ok(())
}
