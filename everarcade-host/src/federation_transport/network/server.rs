use super::{decode_frame, decode_message};
use execution_core::federation_runtime::FederationProtocolMessage;

pub fn server_receive(frame: &[u8]) -> Option<FederationProtocolMessage> {
    let payload = decode_frame(frame)?;
    let envelope = decode_message(&payload)?;
    Some(envelope.message)
}
