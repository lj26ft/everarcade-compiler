use everarcade_host::protocol::{
    envelope::ProtocolEnvelope, message_type::MessageType, protocol_validation::validate_envelope,
};
#[test]
fn protocol_envelope_validation() {
    assert!(validate_envelope(&ProtocolEnvelope {
        version: 1,
        message_type: MessageType::Receipt,
        payload: vec![1]
    }));
}
