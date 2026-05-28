#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recovery;

impl Recovery {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
