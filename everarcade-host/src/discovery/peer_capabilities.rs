#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerCapabilities {
    pub supports_checkpoint_sync: bool,
    pub supports_replay_sync: bool,
}
