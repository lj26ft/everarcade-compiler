use crate::receipt_runtime::execution_receipt::ExecutionReceipt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayWindow {
    pub start_index: u64,
    pub end_index: u64,
    pub receipts: Vec<ExecutionReceipt>,
}

pub fn validate_replay_window(window: &ReplayWindow) -> bool {
    if window.receipts.is_empty() {
        return window.start_index == window.end_index;
    }
    window
        .receipts
        .first()
        .is_some_and(|r| r.timestamp_index == window.start_index)
        && window
            .receipts
            .last()
            .is_some_and(|r| r.timestamp_index == window.end_index)
}
