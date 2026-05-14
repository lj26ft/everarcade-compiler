pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SyncRequestMessage {
    pub from_replay_root: Hash,
    pub target_replay_root: Option<Hash>,
}
