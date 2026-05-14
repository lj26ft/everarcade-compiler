use everarcade_host::protocol::message_signature_validation::message_signature_is_present;
#[test]
fn protocol_signature_required() {
    assert!(message_signature_is_present([8; 32]));
    assert!(!message_signature_is_present([0; 32]));
}
