use crate::distributed_receipts::{
    receipt_manifest::DistributedReceiptManifest, receipt_scan::rebuild_manifest_from_receipts,
    receipt_store_error::ReceiptStoreError,
};
use std::path::Path;
pub fn recover_distributed_receipt_manifest(
    base: impl AsRef<Path>,
) -> Result<DistributedReceiptManifest, ReceiptStoreError> {
    rebuild_manifest_from_receipts(base)
}
