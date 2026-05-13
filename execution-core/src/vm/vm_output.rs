use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VmExecutionOutput {
    pub vm_receipt_root: Hash,
    pub execution_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
    pub external_anchor_root: Hash,
}
