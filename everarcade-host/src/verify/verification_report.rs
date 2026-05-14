#[derive(Clone, Debug)]
pub struct VerificationReport {
    pub package_valid: bool,
    pub receipt_valid: bool,
    pub checkpoint_valid: bool,
    pub anchor_valid: bool,
}

impl VerificationReport {
    pub fn all_valid(&self) -> bool {
        self.package_valid && self.receipt_valid && self.checkpoint_valid && self.anchor_valid
    }
}
