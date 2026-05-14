use std::collections::BTreeMap;

use super::receipt_store::{DistributedExecutionReceipt, Hash};

#[derive(Default)]
pub struct ReceiptIndex {
    by_partition: BTreeMap<Hash, Hash>,
}

impl ReceiptIndex {
    pub fn index(&mut self, receipt: &DistributedExecutionReceipt) {
        self.by_partition
            .insert(receipt.partition_root, receipt.receipt_root);
    }
    pub fn receipt_for_partition(&self, partition_root: &Hash) -> Option<Hash> {
        self.by_partition.get(partition_root).copied()
    }
}
