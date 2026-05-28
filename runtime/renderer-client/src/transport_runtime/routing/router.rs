#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Router;

impl Router {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
