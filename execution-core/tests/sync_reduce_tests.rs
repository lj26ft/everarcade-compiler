use execution_core::{
    receipt_runtime::execution_receipt::ExecutionReceipt,
    sync::{sync_reduce::reduce_sync_exchange, SyncRequest, SyncResponse, SyncStatus},
};

#[test]
fn reduces_valid_exchange() {
    let local = SyncStatus {
        state_root: [0; 32],
        replay_root: [0; 32],
        receipt_root: [0; 32],
        next_index: 10,
    };
    let request = SyncRequest {
        local_state_root: [0; 32],
        local_replay_root: [1; 32],
        local_receipt_root: [0; 32],
        from_index: 0,
        to_index: Some(0),
    };
    let response = SyncResponse {
        checkpoint: None,
        receipts: vec![ExecutionReceipt {
            receipt_id: "r1".into(),
            parent_receipt: None,
            execution_root: "e".into(),
            state_root: "s".into(),
            graph_root: "g".into(),
            replay_root: "rp".into(),
            timestamp_index: 0,
        }],
        state_proofs: vec![],
        receipt_proofs: vec![],
        replay_proof: None,
    };
    let out = reduce_sync_exchange(local, request, response);
    assert!(!out.converged);
}
