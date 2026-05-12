use execution_core::epoch::{epoch_boundary::build_epoch_boundary, epoch_transition::validate_epoch_transition};

#[test]
fn epoch_transition_preserves_replay_continuity() {
    let a = build_epoch_boundary(0, [0;32], [1;32], [2;32], [3;32], [4;32]);
    let b = build_epoch_boundary(1, [1;32], [5;32], [6;32], [7;32], [8;32]);
    assert!(validate_epoch_transition(&a, &b));
}
