use crate::merkle::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncStatus {
    pub state_root: Hash,
    pub replay_root: Hash,
    pub receipt_root: Hash,
    pub next_index: u64,
}
