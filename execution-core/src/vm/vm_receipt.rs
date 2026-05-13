use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmExecutionReceipt {
    pub receipt_id: Hash,
    pub package_root: Hash,
    pub prior_replay_root: Hash,
    pub next_replay_root: Hash,
    pub execution_root: Hash,
    pub checkpoint_root: Hash,
    pub anchor_root: Hash,
}

pub fn compute_vm_receipt_root(receipt: &VmExecutionReceipt) -> Hash {
    let encoded = bincode::serialize(receipt).unwrap_or_default();
    Sha256::digest(encoded).into()
}

pub fn validate_vm_receipt(receipt: &VmExecutionReceipt) -> bool {
    let mut canonical = receipt.clone();
    canonical.receipt_id = [0; 32];
    let root = compute_vm_receipt_root(&canonical);
    root == receipt.receipt_id
}
