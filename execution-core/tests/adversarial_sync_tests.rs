use execution_core::{
    receipt_runtime::execution_receipt::ExecutionReceipt,
    simulation::{adversarial_simulation::simulate_adversarial_rejection, node::SimulatedNode},
    sync::{SyncRequest, SyncResponse, SyncStatus},
};
#[test]
fn replay_mismatch_rejected() {
    let n = SimulatedNode {
        node_id: [9; 32],
        sync_status: SyncStatus {
            state_root: [0; 32],
            replay_root: [0; 32],
            receipt_root: [0; 32],
            next_index: 1,
        },
        checkpoint: None,
        receipts: vec![],
    };
    let receipts = vec![ExecutionReceipt {
        receipt_id: "r2".into(),
        parent_receipt: Some("not-parent".into()),
        execution_root: "e".into(),
        state_root: "s".into(),
        graph_root: "g".into(),
        replay_root: "rr".into(),
        timestamp_index: 0,
    }];
    let bad = simulate_adversarial_rejection(
        &n,
        SyncRequest {
            local_state_root: [0; 32],
            local_replay_root: [8; 32],
            local_receipt_root: [0; 32],
            from_index: 0,
            to_index: None,
        },
        SyncResponse {
            checkpoint: None,
            receipts,
            state_proofs: vec![],
            receipt_proofs: vec![],
            replay_proof: None,
        },
    );
    assert!(bad);
}
