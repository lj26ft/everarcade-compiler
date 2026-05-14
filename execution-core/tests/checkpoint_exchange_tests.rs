use execution_core::{
    checkpoint::{
        checkpoint_root::compute_checkpoint_root, checkpoint_snapshot::CheckpointSnapshot,
    },
    replay::replay_proof::ReplayProof,
    sync::CheckpointExchange,
};

#[test]
fn checkpoint_exchange_struct_roundtrip() {
    let mut cp = CheckpointSnapshot {
        checkpoint_root: [0; 32],
        state_root: [1; 32],
        receipt_root: [2; 32],
        replay_root: [3; 32],
        last_receipt_hash: [4; 32],
        logical_index: 0,
        encoded_state: vec![1, 2, 3],
    };
    cp.checkpoint_root = compute_checkpoint_root(&cp);
    let ex = CheckpointExchange {
        snapshot: cp.clone(),
        continuation_receipts: vec![],
        replay_proof: ReplayProof {
            receipt_root: [0; 32],
            leaf_hash: [0; 32],
            inclusion_proof: execution_core::merkle::inclusion_proof::InclusionProof {
                leaf_index: 0,
                leaf_count: 1,
                siblings: vec![],
            },
        },
    };
    assert_eq!(ex.snapshot, cp);
}
