use super::execution_receipt::ExecutionReceipt;
pub fn compute_receipt_root(receipts: &[ExecutionReceipt]) -> String {
    crate::hash_runtime::root::combine_roots(
        &receipts
            .iter()
            .map(|r| r.receipt_id.clone())
            .collect::<Vec<_>>(),
    )
}
