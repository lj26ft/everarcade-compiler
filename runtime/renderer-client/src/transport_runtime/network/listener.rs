#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Listener;

impl Listener {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
