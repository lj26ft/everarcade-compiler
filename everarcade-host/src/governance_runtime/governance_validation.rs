use super::execution_window::GovernanceExecutionWindow;

pub fn governance_window_valid(window: &GovernanceExecutionWindow) -> bool {
    window.window_root != [0; 32]
        && window.proposal_root != [0; 32]
        && window.quorum_root != [0; 32]
        && window.execution_scope_root != [0; 32]
}
