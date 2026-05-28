#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Backpressure;

impl Backpressure {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
