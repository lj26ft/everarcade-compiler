use execution_core::sync::SyncRequest;

#[test]
fn sync_request_is_deterministic_data() {
    let r1 = SyncRequest { local_state_root:[1;32], local_replay_root:[2;32], local_receipt_root:[3;32], from_index:0, to_index:Some(10)};
    let r2 = r1.clone();
    assert_eq!(r1, r2);
}
