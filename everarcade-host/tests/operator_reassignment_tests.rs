use everarcade_host::{
    operator_recovery::reassignment::reassign_on_failure,
    task_coordination::task_assignment::TaskAssignment,
};

#[test]
fn operator_failure_reassignment_preserves_task_root() {
    let assignment = TaskAssignment {
        assignment_id: [1; 32],
        task_root: [2; 32],
        assigned_operator: [3; 32],
        parent_assignment: None,
    };
    let reassigned = reassign_on_failure(&assignment, [4; 32]);
    assert_eq!(reassigned.task_root, assignment.task_root);
    assert_eq!(reassigned.assigned_operator, [4; 32]);
}
