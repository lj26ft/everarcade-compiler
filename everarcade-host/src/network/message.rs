#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkMessage {
    pub topic: String,
    pub payload: Vec<u8>,
}
