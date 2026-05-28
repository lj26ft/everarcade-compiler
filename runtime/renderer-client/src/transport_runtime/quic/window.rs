#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuicReplayWindow {
    pub continuity_root: String,
    pub ordered: bool,
}

impl QuicReplayWindow {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            continuity_root: continuity_root.into(),
            ordered: true,
        }
    }

    pub fn validates(&self) -> bool {
        self.ordered && !self.continuity_root.is_empty()
    }
}
