#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Runtime;

impl Runtime {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
