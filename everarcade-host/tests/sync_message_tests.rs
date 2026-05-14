use everarcade_host::protocol::sync_request_message::SyncRequestMessage;

#[test]
fn malformed_sync_message_rejected() {
    let bad = b"not-json";
    let parsed: Result<SyncRequestMessage, _> = serde_json::from_slice(bad);
    assert!(parsed.is_err());
}
