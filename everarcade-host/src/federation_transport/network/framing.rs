pub const MAX_FRAME_BYTES: usize = 4 * 1024 * 1024;

pub fn encode_frame(payload: &[u8]) -> Option<Vec<u8>> {
    if payload.len() > MAX_FRAME_BYTES {
        return None;
    }
    let mut out = Vec::with_capacity(4 + payload.len());
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(payload);
    Some(out)
}

pub fn decode_frame(buffer: &[u8]) -> Option<Vec<u8>> {
    if buffer.len() < 4 {
        return None;
    }
    let len = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
    if len > MAX_FRAME_BYTES || buffer.len() != len + 4 {
        return None;
    }
    Some(buffer[4..].to_vec())
}
