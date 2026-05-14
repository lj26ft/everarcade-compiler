use everarcade_host::protocol::message_validation::validate_message;

#[test]
fn malformed_payload_rejected() {
    assert!(validate_message(&[]).is_err());
}
