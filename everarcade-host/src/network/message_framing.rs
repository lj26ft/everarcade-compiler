pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerMessage {
    pub message_id: Hash,
    pub message_type: String,
    pub payload_root: Hash,
    pub payload_bytes: Vec<u8>,
}

pub fn frame_message(message: &PeerMessage) -> Vec<u8> {
    let mut framed = Vec::with_capacity(4 + message.payload_bytes.len());
    let len = message.payload_bytes.len() as u32;
    framed.extend_from_slice(&len.to_be_bytes());
    framed.extend_from_slice(&message.payload_bytes);
    framed
}

pub fn deframe_payload(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() < 4 {
        return None;
    }
    let mut len_bytes = [0_u8; 4];
    len_bytes.copy_from_slice(&bytes[..4]);
    let payload_len = u32::from_be_bytes(len_bytes) as usize;
    if bytes.len() != 4 + payload_len {
        return None;
    }
    Some(bytes[4..].to_vec())
}
