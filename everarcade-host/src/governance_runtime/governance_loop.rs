use super::{continuity_validation::validate_governance_continuity, execution_commit::GovernanceExecutionCommit, proposal_execution::execute_proposal, quorum_execution::resolve_quorum, window_execution::derive_execution_window_root};
pub type Hash = [u8; 32];

pub fn execute_governance_loop(proposal_root: Hash, scope_root: Hash, vote_root: Hash, quorum_threshold: u8, replay_root: Hash, checkpoint_root: Hash) -> Result<GovernanceExecutionCommit, &'static str> {
    let proposal_execution_root = execute_proposal(proposal_root, scope_root);
    let quorum_root = resolve_quorum(vote_root, quorum_threshold)?;
    let execution_window_root = derive_execution_window_root(proposal_execution_root, quorum_root);
    let commit = GovernanceExecutionCommit { proposal_root: proposal_execution_root, quorum_root, execution_window_root, replay_root, checkpoint_root };
    validate_governance_continuity(&commit, replay_root, checkpoint_root)?;
    Ok(commit)
}
