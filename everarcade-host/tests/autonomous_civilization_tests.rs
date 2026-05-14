use everarcade_host::{
    civilization_runtime::runtime_loop::execute_runtime_loop,
    governance_runtime::governance_loop::execute_governance_loop,
};
#[test]
fn autonomous_loop_binds_governance_and_civilization() {
    let g = execute_governance_loop([1; 32], [2; 32], [3; 32], 1, [4; 32], [5; 32]).unwrap();
    let c = execute_runtime_loop(
        g.execution_window_root,
        [8; 32],
        g.replay_root,
        g.checkpoint_root,
    );
    assert_eq!(c.replay_root, [4; 32]);
}
