pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PeerStatusMessage {
    pub package_root: Hash,
    pub latest_receipt_root: Hash,
    pub latest_checkpoint_root: Hash,
    pub replay_root: Hash,
}
