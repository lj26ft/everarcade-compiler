use super::execution_receipt::ExecutionReceipt;
pub fn validate_chain(receipts: &[ExecutionReceipt]) -> bool { receipts.windows(2).all(|w| w[1].parent_receipt.as_ref() == Some(&w[0].receipt_id)) }
