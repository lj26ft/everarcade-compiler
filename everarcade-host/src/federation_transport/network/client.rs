use super::{decode_frame, decode_message, encode_frame, encode_message};
use execution_core::federation_runtime::FederationProtocolMessage;

pub fn client_roundtrip(message: FederationProtocolMessage) -> Option<FederationProtocolMessage> {
    let bytes = encode_message(message)?;
    let frame = encode_frame(1, &bytes)?;
    let (_, payload) = decode_frame(&frame)?;
    Some(decode_message(&payload)?.message)
}
