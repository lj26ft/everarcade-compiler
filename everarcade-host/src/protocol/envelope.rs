use super::message_type::MessageType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolEnvelope {
    pub version: u16,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}
