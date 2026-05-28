#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Connection;

impl Connection {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
