use super::governance::GovernanceAction;

pub fn propagate_authority(actions: &[GovernanceAction]) -> usize {
    actions.len()
}
