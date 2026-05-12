#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityStamp {
    pub epoch: u32,
    pub runtime_version: u32,
    pub trace_version: u32,
    pub proof_version: u32,
}

impl CompatibilityStamp {
    pub fn is_compatible_with(self, other: Self) -> bool {
        self == other
    }
}
