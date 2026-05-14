use everarcade_host::{
    governance_runtime::governance_loop::execute_governance_loop,
    window::{
        window_checkpoint::derive_window_checkpoint_root,
        window_divergence::detect_window_divergence,
        window_execution::derive_window_execution_root, window_replay::derive_window_replay_root,
    },
};
#[test]
fn end_to_end_governance_flow() {
    let g = execute_governance_loop([1; 32], [2; 32], [3; 32], 1, [4; 32], [5; 32]).unwrap();
    let w = derive_window_execution_root(1, g.execution_window_root);
    let r = derive_window_replay_root(g.replay_root, w);
    let c = derive_window_checkpoint_root(r, 9);
    assert!(detect_window_divergence(c, c).is_ok());
}
