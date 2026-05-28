use super::node::BehaviorNode;
pub fn order_nodes(mut nodes: Vec<BehaviorNode>) -> Vec<BehaviorNode> {
    nodes.sort_by(|a, b| a.priority.cmp(&b.priority).then_with(|| a.id.cmp(&b.id)));
    nodes
}
