use super::envelope::ProtocolEnvelope;

pub fn validate_envelope(e: &ProtocolEnvelope) -> bool {
    e.version > 0 && !e.payload.is_empty()
}
