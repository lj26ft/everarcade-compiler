use execution_core::vm::VmExecutionReceipt;
pub fn export_roots(receipts: &[VmExecutionReceipt]) -> Vec<[u8; 32]> {
    receipts.iter().map(|r| r.receipt_id).collect()
}
