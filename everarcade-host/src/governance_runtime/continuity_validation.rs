use super::execution_commit::GovernanceExecutionCommit;
pub fn validate_governance_continuity(
    commit: &GovernanceExecutionCommit,
    expected_replay_root: [u8; 32],
    expected_checkpoint_root: [u8; 32],
) -> Result<(), &'static str> {
    if commit.replay_root != expected_replay_root {
        return Err("governance replay divergence");
    }
    if commit.checkpoint_root != expected_checkpoint_root {
        return Err("governance checkpoint divergence");
    }
    Ok(())
}
