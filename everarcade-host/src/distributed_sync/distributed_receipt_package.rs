use crate::distributed_sync::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedReceiptPackage {
    pub package_root: Hash,
    pub partition_root: Hash,
    pub receipt_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
    pub receipt_bytes: Vec<u8>,
}
