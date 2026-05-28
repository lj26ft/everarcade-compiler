#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Deployment;

impl Deployment {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
