#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerCapability {
    pub transport: String,
    pub replay_only: bool,
}
