use execution_core::sync::{sync_apply::apply_sync_exchange, SyncRequest, SyncResponse, SyncStatus};

#[test]
fn apply_rejects_bad_request() {
    let res = apply_sync_exchange(
        SyncStatus { state_root:[0;32], replay_root:[0;32], receipt_root:[0;32], next_index:1 },
        SyncRequest { local_state_root:[0;32], local_replay_root:[0;32], local_receipt_root:[0;32], from_index:3, to_index:None },
        SyncResponse { checkpoint:None, receipts:vec![], state_proofs:vec![], receipt_proofs:vec![], replay_proof:None }
    );
    assert!(res.failure.is_some());
}
