use execution_core::sync::{
    sync_apply::apply_sync_exchange, SyncRequest, SyncResponse, SyncStatus,
};
#[test]
fn deterministic_empty_sync() {
    let local = SyncStatus {
        state_root: [1; 32],
        replay_root: [2; 32],
        receipt_root: [3; 32],
        next_index: 0,
    };
    let req = SyncRequest {
        local_state_root: [1; 32],
        local_replay_root: [2; 32],
        local_receipt_root: [3; 32],
        from_index: 0,
        to_index: None,
    };
    let rsp = SyncResponse {
        checkpoint: None,
        receipts: vec![],
        state_proofs: vec![],
        receipt_proofs: vec![],
        replay_proof: None,
    };
    assert_eq!(
        apply_sync_exchange(local.clone(), req.clone(), rsp.clone()),
        apply_sync_exchange(local, req, rsp)
    );
}
