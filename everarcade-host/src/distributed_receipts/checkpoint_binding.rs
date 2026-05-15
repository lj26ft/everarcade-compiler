use crate::distributed_receipts::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptCheckpointBinding {
    pub partition_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
}
