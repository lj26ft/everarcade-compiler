use crate::simulation::node::SimulatedNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeState {
    pub node: SimulatedNode,
    pub accepted: bool,
}
