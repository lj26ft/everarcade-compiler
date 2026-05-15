use crate::distributed_receipts::{
    execution_receipt::DistributedExecutionReceipt, receipt_codec::encode_canonical,
    receipt_store_error::ReceiptStoreError, Hash,
};
use crate::distributed_sync::distributed_receipt_package::DistributedReceiptPackage;

pub fn export_distributed_receipt(
    receipt: &DistributedExecutionReceipt,
    partition_root: Hash,
    replay_root: Hash,
    checkpoint_root: Hash,
) -> Result<DistributedReceiptPackage, ReceiptStoreError> {
    Ok(DistributedReceiptPackage {
        package_root: receipt.package_root,
        partition_root,
        receipt_root: receipt.receipt_root,
        replay_root,
        checkpoint_root,
        receipt_bytes: encode_canonical(receipt)?,
    })
}
