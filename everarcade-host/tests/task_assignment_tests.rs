use everarcade_host::task_coordination::task_scheduler::deterministic_assignment;

#[test]
fn deterministic_assignment_is_stable() {
    let operators = vec![[7; 32], [8; 32], [9; 32]];
    let a1 = deterministic_assignment([1; 32], &operators, Some([2; 32]));
    let a2 = deterministic_assignment([1; 32], &operators, Some([2; 32]));
    assert_eq!(a1, a2);
}
