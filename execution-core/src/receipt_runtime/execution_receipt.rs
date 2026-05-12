#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub receipt_id: String,
    pub parent_receipt: Option<String>,
    pub execution_root: String,
    pub state_root: String,
    pub graph_root: String,
    pub replay_root: String,
    pub timestamp_index: u64,
}
