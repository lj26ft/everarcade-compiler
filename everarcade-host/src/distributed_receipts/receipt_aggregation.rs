use sha2::{Digest, Sha256};

use super::execution_receipt::{DistributedExecutionReceipt, Hash};

pub fn aggregate_receipt_root(receipts: &[DistributedExecutionReceipt]) -> Hash {
    let mut hasher = Sha256::new();
    for receipt in receipts {
        hasher.update(receipt.receipt_root);
    }
    hasher.finalize().into()
}
