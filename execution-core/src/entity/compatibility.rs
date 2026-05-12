#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatibilityMatrix {
    pub epoch_compatible: bool,
    pub runtime_compatible: bool,
    pub proof_compatible: bool,
    pub lineage_compatible: bool,
}

impl CompatibilityMatrix {
    pub fn is_compatible(&self) -> bool {
        self.epoch_compatible && self.runtime_compatible && self.proof_compatible && self.lineage_compatible
    }
}
