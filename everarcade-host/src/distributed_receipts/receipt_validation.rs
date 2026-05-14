use super::execution_receipt::DistributedExecutionReceipt;

pub fn validate_receipt(receipt: &DistributedExecutionReceipt) -> bool {
    receipt.receipt_root != [0u8; 32]
        && receipt.task_root != [0u8; 32]
        && receipt.package_root != [0u8; 32]
}
