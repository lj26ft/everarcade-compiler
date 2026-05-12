use crate::merkle::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointSnapshot {
    pub checkpoint_root: Hash,
    pub state_root: Hash,
    pub receipt_root: Hash,
    pub replay_root: Hash,
    pub last_receipt_hash: Hash,
    pub logical_index: u64,
    pub encoded_state: Vec<u8>,
}
