pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryPlan {
    pub latest_checkpoint_root: Hash,
    pub missing_receipts: u64,
    pub requires_checkpoint_import: bool,
}
