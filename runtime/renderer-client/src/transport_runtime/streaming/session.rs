#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Session;

impl Session {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
