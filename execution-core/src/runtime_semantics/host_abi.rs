#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostFrame {
    pub input_len: u32,
    pub payload: Vec<u8>,
}

impl HostFrame {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { input_len: payload.len() as u32, payload }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = self.input_len.to_le_bytes().to_vec();
        out.extend_from_slice(&self.payload);
        out
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 4 { return None; }
        let len = u32::from_le_bytes(bytes[0..4].try_into().ok()?) as usize;
        let payload = bytes.get(4..4 + len)?.to_vec();
        Some(Self { input_len: len as u32, payload })
    }
}
