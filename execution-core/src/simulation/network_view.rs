use crate::simulation::node::SimulatedNode;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NetworkView {
    pub nodes: Vec<SimulatedNode>,
}
