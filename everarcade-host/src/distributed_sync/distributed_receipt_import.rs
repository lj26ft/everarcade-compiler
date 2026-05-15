use crate::distributed_receipts::{
    disk_store::DistributedReceiptDiskStore, receipt_store_error::ReceiptStoreError,
};
use crate::distributed_sync::{
    distributed_receipt_package::DistributedReceiptPackage,
    distributed_receipt_validation::validate_receipt_package,
};

pub fn import_distributed_receipt(
    store: &DistributedReceiptDiskStore,
    package: &DistributedReceiptPackage,
) -> Result<(), ReceiptStoreError> {
    let receipt = validate_receipt_package(package)?;
    store.persist_receipt(package.replay_root, package.checkpoint_root, &receipt)
}
