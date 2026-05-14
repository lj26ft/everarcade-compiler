pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuorumMessage {
    pub root: Hash,
}
