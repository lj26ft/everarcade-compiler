use everarcade_host::governance_runtime::{
    execution_window::GovernanceExecutionWindow, governance_scheduler::schedule_governance_window,
    governance_validation::governance_window_valid,
};

#[test]
fn governance_runtime_window_is_deterministic() {
    let a = GovernanceExecutionWindow {
        window_root: [2; 32],
        proposal_root: [1; 32],
        quorum_root: [3; 32],
        execution_scope_root: [4; 32],
    };
    let b = GovernanceExecutionWindow {
        window_root: [1; 32],
        proposal_root: [5; 32],
        quorum_root: [6; 32],
        execution_scope_root: [7; 32],
    };
    let ordered = schedule_governance_window(&[a.clone()], b.clone());
    assert_eq!(ordered[0].window_root, b.window_root);
    assert_eq!(ordered[1].window_root, a.window_root);
    assert!(ordered.iter().all(governance_window_valid));
}
