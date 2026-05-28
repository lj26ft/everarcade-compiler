#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpReplayRecovery {
    pub last_acknowledged: u64,
    pub continuity_root: String,
}
impl TcpReplayRecovery {
    pub fn reconnect_from(last_acknowledged: u64, continuity_root: impl Into<String>) -> Self {
        Self {
            last_acknowledged,
            continuity_root: continuity_root.into(),
        }
    }
}
