use crate::network::node::NetworkNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingDecision {
    pub package_id: String,
    pub selected_node_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct RoutingTable;

impl RoutingTable {
    pub fn select_execution_node(
        &self,
        package_id: &str,
        epoch: u64,
        nodes: &[NetworkNode],
    ) -> Option<RoutingDecision> {
        let mut eligible: Vec<&NetworkNode> = nodes
            .iter()
            .filter(|n| n.execution && n.supports_epoch(epoch))
            .collect();
        eligible.sort_by(|a, b| b.reputation_score.cmp(&a.reputation_score).then(a.node_id.cmp(&b.node_id)));

        eligible.first().map(|n| RoutingDecision {
            package_id: package_id.to_string(),
            selected_node_id: n.node_id.clone(),
        })
    }
}
