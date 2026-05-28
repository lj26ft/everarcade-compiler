#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ObserverReconnect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveObserverReconnect {
    pub next_sequence: u64,
    pub checkpoint: String,
}
impl LiveObserverReconnect {
    pub fn from_cursor(next_sequence: u64, checkpoint: impl Into<String>) -> Self {
        Self {
            next_sequence,
            checkpoint: checkpoint.into(),
        }
    }
}
