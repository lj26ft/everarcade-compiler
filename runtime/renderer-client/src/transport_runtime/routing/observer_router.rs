#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObserverRouter;

impl ObserverRouter {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
