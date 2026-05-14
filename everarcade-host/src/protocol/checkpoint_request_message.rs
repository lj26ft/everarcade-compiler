pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CheckpointRequestMessage {
    pub checkpoint_root: Option<Hash>,
}
