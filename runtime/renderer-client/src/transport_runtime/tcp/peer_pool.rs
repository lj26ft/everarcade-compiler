#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpReplayPeerPool {
    pub continuity_root: String,
    pub replay_only: bool,
}

impl TcpReplayPeerPool {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            continuity_root: continuity_root.into(),
            replay_only: true,
        }
    }

    pub fn preserves_ordering(&self) -> bool {
        self.replay_only && !self.continuity_root.is_empty()
    }
}
