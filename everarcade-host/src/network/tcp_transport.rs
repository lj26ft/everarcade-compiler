use super::message_framing::PeerMessage;

#[derive(Clone, Debug)]
pub struct TcpTransport {
    pub bind_addr: String,
}

impl TcpTransport {
    pub fn new(bind_addr: impl Into<String>) -> Self {
        Self {
            bind_addr: bind_addr.into(),
        }
    }

    pub fn send(&self, _peer: &str, _message: &PeerMessage) -> Result<(), String> {
        Ok(())
    }
}
