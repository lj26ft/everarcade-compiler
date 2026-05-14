pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointDelta {
    pub from_checkpoint_root: Hash,
    pub to_checkpoint_root: Hash,
    pub missing_receipts: u64,
}
