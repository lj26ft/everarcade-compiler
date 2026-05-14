use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DistributedExecutionReceipt {
    pub receipt_root: Hash,
    pub partition_root: Hash,
    pub execution_root: Hash,
    pub replay_root: Hash,
}

impl DistributedExecutionReceipt {
    pub fn new(partition_root: Hash, execution_root: Hash, replay_root: Hash) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(partition_root);
        hasher.update(execution_root);
        hasher.update(replay_root);
        let receipt_root: Hash = hasher.finalize().into();
        Self {
            receipt_root,
            partition_root,
            execution_root,
            replay_root,
        }
    }
}

#[derive(Default)]
pub struct ReceiptStore {
    receipts: BTreeMap<Hash, DistributedExecutionReceipt>,
}

impl ReceiptStore {
    pub fn put(&mut self, receipt: DistributedExecutionReceipt) {
        self.receipts.insert(receipt.receipt_root, receipt);
    }
    pub fn get(&self, receipt_root: &Hash) -> Option<&DistributedExecutionReceipt> {
        self.receipts.get(receipt_root)
    }
    pub fn all(&self) -> Vec<DistributedExecutionReceipt> {
        self.receipts.values().cloned().collect()
    }
}
