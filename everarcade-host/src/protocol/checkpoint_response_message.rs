pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CheckpointResponseMessage {
    pub checkpoint_root: Hash,
    pub bytes: Vec<u8>,
}
