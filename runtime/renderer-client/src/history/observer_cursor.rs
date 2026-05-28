#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObserverReplayCursor {
    pub next_sequence: u64,
    pub checkpoint: String,
}
