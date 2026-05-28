#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TopologyRouter;

impl TopologyRouter {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
