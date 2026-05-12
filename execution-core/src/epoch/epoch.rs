use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEpoch {
    pub epoch_index: u64,
    pub start_receipt: Hash,
    pub end_receipt: Hash,
    pub epoch_root: Hash,
    pub checkpoint_root: Hash,
    pub replay_root: Hash,
    pub receipt_root: Hash,
}
