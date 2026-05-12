use crate::merkle::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncRequest {
    pub local_state_root: Hash,
    pub local_replay_root: Hash,
    pub local_receipt_root: Hash,
    pub from_index: u64,
    pub to_index: Option<u64>,
}
