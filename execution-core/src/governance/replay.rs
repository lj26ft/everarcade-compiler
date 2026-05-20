use super::{
    error::GovernanceError, verification::verify_governance_integrity, GovernanceContinuity,
};
pub fn replay_governance_lineage(input: &GovernanceContinuity) -> GovernanceContinuity {
    let mut out = input.clone();
    out.votes.sort();
    out
}
pub fn verify_governance_replay(input: &GovernanceContinuity) -> Result<(), GovernanceError> {
    let replayed = replay_governance_lineage(input);
    if &replayed != input {
        return verify_governance_integrity(&replayed);
    }
    verify_governance_integrity(input)
}
