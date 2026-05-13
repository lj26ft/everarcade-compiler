use super::message::NetworkMessage;

pub trait NetworkTransport {
    fn send(&self, _msg: &NetworkMessage) -> Result<(), String>;
}
