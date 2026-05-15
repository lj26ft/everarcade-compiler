use crate::distributed_sync::{distributed_receipt_package::DistributedReceiptPackage, Hash};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedReceiptResponse {
    pub package_root: Hash,
    pub receipts: Vec<DistributedReceiptPackage>,
    pub final_replay_root: Hash,
}
