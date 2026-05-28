#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketObserverRecovery {
    pub cursor: u64,
    pub checkpoint: String,
}
impl WebSocketObserverRecovery {
    pub fn resume(cursor: u64, checkpoint: impl Into<String>) -> Self {
        Self {
            cursor,
            checkpoint: checkpoint.into(),
        }
    }
}
