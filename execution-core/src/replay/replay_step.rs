#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceStep {
    pub logical_index: usize,
    pub receipt_hash: String,
    pub parent_receipt_hash: Option<String>,
    pub prior_state_root: String,
    pub transition_root: String,
    pub next_state_root: String,
    pub replay_root: String,
}
