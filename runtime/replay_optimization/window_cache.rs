#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWindowCache {
    pub continuity_root: String,
    pub replay_only: bool,
    pub reconstruction_only: bool,
}

impl ReplayWindowCache {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            continuity_root: continuity_root.into(),
            replay_only: true,
            reconstruction_only: true,
        }
    }

    pub fn validates(&self) -> bool {
        !self.continuity_root.is_empty() && self.replay_only && self.reconstruction_only
    }

    pub fn rejects_authority_mutation(&self) -> bool {
        true
    }
}
