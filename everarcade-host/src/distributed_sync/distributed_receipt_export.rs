use crate::distributed_receipts::{
    execution_receipt::DistributedExecutionReceipt, receipt_codec::encode_canonical,
    receipt_store_error::ReceiptStoreError, Hash,
};
use crate::distributed_sync::distributed_receipt_package::DistributedReceiptPackage;
use sha2::{Digest, Sha256};

pub fn export_distributed_receipt(
    receipt: &DistributedExecutionReceipt,
    partition_root: Hash,
    _replay_root: Hash,
    checkpoint_root: Hash,
) -> Result<DistributedReceiptPackage, ReceiptStoreError> {
    let mut hasher = Sha256::new();
    hasher.update(receipt.receipt_root);
    hasher.update(partition_root);
    hasher.update(checkpoint_root);
    let replay_root: Hash = hasher.finalize().into();

    Ok(DistributedReceiptPackage {
        package_root: receipt.package_root,
        partition_root,
        receipt_root: receipt.receipt_root,
        replay_root,
        checkpoint_root,
        receipt_bytes: encode_canonical(receipt)?,
    })
}
