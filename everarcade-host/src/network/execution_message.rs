pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionMessage {
    pub window_id: Hash,
    pub execution_root: Hash,
}
