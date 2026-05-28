#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverPartition {
    pub continuity_root: String,
    pub reconstruction_only: bool,
}

impl ObserverPartition {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            continuity_root: continuity_root.into(),
            reconstruction_only: true,
        }
    }

    pub fn rejects_authority_mutation(&self) -> bool {
        self.reconstruction_only && !self.continuity_root.is_empty()
    }
}
