use crate::distributed_receipts::{
    execution_receipt::DistributedExecutionReceipt, receipt_codec::decode_canonical,
    receipt_store_error::ReceiptStoreError,
};
use crate::distributed_sync::distributed_receipt_package::DistributedReceiptPackage;

pub fn validate_receipt_package(
    package: &DistributedReceiptPackage,
) -> Result<DistributedExecutionReceipt, ReceiptStoreError> {
    let receipt = decode_canonical(&package.receipt_bytes)?;
    if receipt.package_root != package.package_root {
        return Err(ReceiptStoreError::Validation("package root mismatch"));
    }
    if receipt.receipt_root != package.receipt_root {
        return Err(ReceiptStoreError::Validation("receipt root mismatch"));
    }
    Ok(receipt)
}
