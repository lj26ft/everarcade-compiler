use sha2::{Digest, Sha256};

pub const PROTOCOL_VERSION: u16 = 1;
pub const HEADER_LEN: usize = 2 + 2 + 4 + 32;
pub const MAX_FRAME_BYTES: usize = 4 * 1024 * 1024;

pub fn encode_frame(message_type: u16, payload: &[u8]) -> Option<Vec<u8>> {
    if payload.len() > MAX_FRAME_BYTES {
        return None;
    }
    let payload_hash: [u8; 32] = Sha256::digest(payload).into();
    let mut out = Vec::with_capacity(HEADER_LEN + payload.len());
    out.extend_from_slice(&PROTOCOL_VERSION.to_be_bytes());
    out.extend_from_slice(&message_type.to_be_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(&payload_hash);
    out.extend_from_slice(payload);
    Some(out)
}

pub fn decode_frame(buffer: &[u8]) -> Option<(u16, Vec<u8>)> {
    if buffer.len() < HEADER_LEN {
        return None;
    }
    let protocol_version = u16::from_be_bytes([buffer[0], buffer[1]]);
    if protocol_version != PROTOCOL_VERSION {
        return None;
    }
    let message_type = u16::from_be_bytes([buffer[2], buffer[3]]);
    let len = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) as usize;
    if len > MAX_FRAME_BYTES || buffer.len() != len + HEADER_LEN {
        return None;
    }
    let provided_hash = &buffer[8..40];
    let payload = &buffer[HEADER_LEN..];
    let computed_hash: [u8; 32] = Sha256::digest(payload).into();
    if provided_hash != computed_hash.as_slice() {
        return None;
    }
    Some((message_type, payload.to_vec()))
}
