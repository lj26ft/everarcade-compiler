use crate::distributed_receipts::{
    disk_store::DistributedReceiptDiskStore, execution_receipt::DistributedExecutionReceipt,
    receipt_store_error::ReceiptStoreError, Hash,
};
use crate::distributed_sync::{
    distributed_receipt_package::DistributedReceiptPackage,
    distributed_receipt_validation::validate_receipt_package,
};
use sha2::{Digest, Sha256};

pub fn import_distributed_receipt(
    store: &DistributedReceiptDiskStore,
    package: &DistributedReceiptPackage,
) -> Result<(), ReceiptStoreError> {
    let receipt = validate_receipt_package(package)?;
    verify_replay_root(&receipt, package)?;
    verify_checkpoint_continuity(store, package)?;
    store.persist_receipt(package.replay_root, package.checkpoint_root, &receipt)
}

fn verify_replay_root(
    receipt: &DistributedExecutionReceipt,
    package: &DistributedReceiptPackage,
) -> Result<(), ReceiptStoreError> {
    let mut hasher = Sha256::new();
    hasher.update(receipt.receipt_root);
    hasher.update(package.partition_root);
    hasher.update(package.checkpoint_root);
    let computed: Hash = hasher.finalize().into();
    if computed != package.replay_root {
        return Err(ReceiptStoreError::Validation(
            "replay verification failed for imported receipt",
        ));
    }
    Ok(())
}

fn verify_checkpoint_continuity(
    store: &DistributedReceiptDiskStore,
    package: &DistributedReceiptPackage,
) -> Result<(), ReceiptStoreError> {
    let manifest = store.load_manifest()?;
    if manifest.receipt_count > 0 {
        match manifest.latest_checkpoint_root {
            Some(root) if root == package.checkpoint_root => {}
            _ => {
                return Err(ReceiptStoreError::Validation(
                    "checkpoint continuity mismatch during import",
                ));
            }
        }
    }
    Ok(())
}
