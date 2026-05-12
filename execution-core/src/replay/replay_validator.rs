use everarcade_abi::ExecutionReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DivergenceReason {
    ParentReceiptMismatch,
    PriorRootMismatch,
    NextRootMismatch,
}

pub fn validate_parent_link(receipts: &[ExecutionReceipt], idx: usize) -> bool {
    if idx == 0 {
        return receipts[idx].previous_snapshot_hash.is_none();
    }
    receipts[idx].previous_snapshot_hash.as_ref() == Some(&receipts[idx - 1].receipt_hash)
}
