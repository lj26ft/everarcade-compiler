use execution_core::sync::{build_sync_plan, SyncAction, SyncRequest, SyncStatus};

#[test]
fn same_request_same_status_same_plan() {
    let req = SyncRequest {
        local_state_root: [1; 32],
        local_replay_root: [2; 32],
        local_receipt_root: [3; 32],
        from_index: 1,
        to_index: Some(5),
    };
    let status = SyncStatus {
        state_root: [4; 32],
        replay_root: [2; 32],
        receipt_root: [5; 32],
        next_index: 10,
    };
    assert_eq!(
        build_sync_plan(req.clone(), status.clone()),
        build_sync_plan(req, status)
    );
}

#[test]
fn replay_match_requests_proof_only() {
    let req = SyncRequest {
        local_state_root: [1; 32],
        local_replay_root: [9; 32],
        local_receipt_root: [3; 32],
        from_index: 1,
        to_index: None,
    };
    let status = SyncStatus {
        state_root: [4; 32],
        replay_root: [9; 32],
        receipt_root: [5; 32],
        next_index: 10,
    };
    assert_eq!(
        build_sync_plan(req, status).action,
        SyncAction::RequestProofOnly
    );
}
