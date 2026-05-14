use execution_core::checkpoint::{
    checkpoint_root::compute_checkpoint_root, checkpoint_snapshot::CheckpointSnapshot,
    checkpoint_validation::validate_checkpoint,
};

#[test]
fn checkpoint_validates_against_recomputed_root() {
    let mut snapshot = CheckpointSnapshot {
        checkpoint_root: [0; 32],
        state_root: [1; 32],
        receipt_root: [2; 32],
        replay_root: [3; 32],
        last_receipt_hash: [4; 32],
        logical_index: 7,
        encoded_state: vec![9, 8, 7],
    };
    snapshot.checkpoint_root = compute_checkpoint_root(&snapshot);
    assert!(validate_checkpoint(&snapshot));
}
