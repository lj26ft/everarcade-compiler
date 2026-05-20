use execution_core::federation_runtime::{
    canonical_serialize, validate_bundle_size, validate_protocol_message,
    FederationProtocolMessage, ProtocolEnvelope, FEDERATION_PROTOCOL_VERSION,
};

pub fn encode_message(message: FederationProtocolMessage) -> Option<Vec<u8>> {
    let env = ProtocolEnvelope {
        version: FEDERATION_PROTOCOL_VERSION,
        message,
    };
    canonical_serialize(&env).ok()
}

pub fn encode_typed_message(
    message_type: u16,
    message: FederationProtocolMessage,
) -> Option<Vec<u8>> {
    let payload = encode_message(message)?;
    super::framing::encode_frame(message_type, &payload)
}

pub fn decode_message(bytes: &[u8]) -> Option<ProtocolEnvelope> {
    validate_bundle_size(bytes).ok()?;
    let env: ProtocolEnvelope = serde_json::from_slice(bytes).ok()?;
    validate_protocol_message(&env).ok()?;
    Some(env)
}

pub fn decode_typed_message(bytes: &[u8]) -> Option<(u16, ProtocolEnvelope)> {
    let (message_type, payload) = super::framing::decode_frame(bytes)?;
    let envelope = decode_message(&payload)?;
    Some((message_type, envelope))
}
