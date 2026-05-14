use everarcade_host::distributed_execution::{
    execution_assignment::ExecutionAssignment, execution_validation::assignment_matches_window,
    execution_window::DistributedExecutionWindow,
};

#[test]
fn distributed_window_assignment_matches_operator() {
    let window = DistributedExecutionWindow {
        window_id: [1; 32],
        package_root: [2; 32],
        assigned_operator: [3; 32],
        execution_root: [4; 32],
        replay_root: [5; 32],
    };
    let assignment = ExecutionAssignment {
        window_id: [1; 32],
        task_root: [9; 32],
        assigned_operator: [3; 32],
    };
    assert!(assignment_matches_window(&window, &assignment));
}
