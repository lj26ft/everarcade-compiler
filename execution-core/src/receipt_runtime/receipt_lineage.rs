use super::execution_receipt::ExecutionReceipt;
pub fn lineage_ids(receipts: &[ExecutionReceipt]) -> Vec<String> {
    receipts.iter().map(|r| r.receipt_id.clone()).collect()
}
