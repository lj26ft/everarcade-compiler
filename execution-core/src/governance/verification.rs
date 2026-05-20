use super::{
    error::GovernanceError, verify_authority_lineage, verify_policy_continuity,
    verify_proposal_lineage, verify_vote_continuity, GovernanceContinuity,
};
pub fn verify_governance_integrity(c: &GovernanceContinuity) -> Result<(), GovernanceError> {
    verify_proposal_lineage(&c.proposals)?;
    verify_vote_continuity(&c.votes)?;
    verify_policy_continuity(&c.policies)?;
    verify_authority_lineage(&c.authorities)?;
    Ok(())
}
pub fn verify_policy_replay(c: &GovernanceContinuity) -> Result<(), GovernanceError> {
    verify_policy_continuity(&c.policies)
}
pub fn verify_authority_continuity(c: &GovernanceContinuity) -> Result<(), GovernanceError> {
    verify_authority_lineage(&c.authorities)
}
pub fn inspect_governance_lineage(c: &GovernanceContinuity) -> usize {
    c.proposals.len() + c.votes.len()
}
pub fn inspect_policy_continuity(c: &GovernanceContinuity) -> usize {
    c.policies.len()
}
pub fn inspect_authority_replay(c: &GovernanceContinuity) -> usize {
    c.authorities.len()
}
