use super::operator_node::OperatorNode;

#[derive(Default)]
pub struct LocalCluster {
    pub nodes: Vec<OperatorNode>,
}

impl LocalCluster {
    pub fn with_nodes(nodes: Vec<OperatorNode>) -> Self {
        Self { nodes }
    }
}
