#[derive(Clone, Debug, Default)]
pub struct RecoveryReport {
    pub orphan_receipts: u64,
    pub orphan_checkpoints: u64,
    pub orphan_anchors: u64,
    pub repaired_manifest: bool,
}
