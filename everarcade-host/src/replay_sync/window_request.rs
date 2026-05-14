use super::replay_window::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowRequest {
    pub start_receipt_root: Hash,
    pub max_receipts: u64,
}
