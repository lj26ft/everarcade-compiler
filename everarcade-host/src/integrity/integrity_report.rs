#[derive(Debug, Default)]
pub struct IntegrityReport {
    pub package_root_ok: bool,
    pub receipt_root_ok: bool,
    pub checkpoint_root_ok: bool,
    pub manifest_root_ok: bool,
    pub anchor_root_ok: bool,
    pub proof_root_ok: bool,
}
impl IntegrityReport {
    pub fn all_passed(&self) -> bool {
        self.package_root_ok
            && self.receipt_root_ok
            && self.checkpoint_root_ok
            && self.manifest_root_ok
            && self.anchor_root_ok
            && self.proof_root_ok
    }
}
