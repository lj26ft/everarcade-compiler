use crate::{
    merkle::{receipt_merkle::receipt_root, Hash},
    receipt_runtime::execution_receipt::ExecutionReceipt,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptRange {
    pub start_index: u64,
    pub end_index: u64,
    pub receipts: Vec<ExecutionReceipt>,
    pub receipt_root: Hash,
}

pub fn validate_receipt_range(range: &ReceiptRange) -> bool {
    if range.receipts.is_empty() {
        return range.start_index == range.end_index && range.receipt_root == receipt_root(&[]);
    }
    if range.receipts.len() as u64 != (range.end_index - range.start_index + 1) {
        return false;
    }
    let mut expected_index = range.start_index;
    for (i, r) in range.receipts.iter().enumerate() {
        if r.timestamp_index != expected_index {
            return false;
        }
        if i > 0 && r.parent_receipt.as_deref() != Some(range.receipts[i - 1].receipt_id.as_str()) {
            return false;
        }
        expected_index += 1;
    }
    receipt_root(&range.receipts) == range.receipt_root
}
