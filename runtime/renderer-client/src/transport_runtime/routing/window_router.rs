#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowRouter;

impl WindowRouter {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
