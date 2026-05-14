use execution_core::checkpoint::{
    checkpoint_restore::restore_checkpoint, checkpoint_root::compute_checkpoint_root,
    checkpoint_snapshot::CheckpointSnapshot,
};

#[test]
fn restore_roundtrip() {
    let mut cp = CheckpointSnapshot {
        checkpoint_root: [0; 32],
        state_root: [1; 32],
        receipt_root: [2; 32],
        replay_root: [3; 32],
        last_receipt_hash: [4; 32],
        logical_index: 1,
        encoded_state: vec![1, 2, 3],
    };
    cp.checkpoint_root = compute_checkpoint_root(&cp);
    let out = restore_checkpoint(&cp).expect("restores");
    assert_eq!(out, vec![1, 2, 3]);
}
