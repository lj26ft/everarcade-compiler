use crate::dag::{ExecutionGraph, ExecutionNode};

pub fn parse_dag_from_bytes(
    bytes: &[u8],
) -> (
    ExecutionGraph,
    std::collections::HashMap<String, serde_json::Value>,
) {
    let plan: crate::ExecutionPlan = serde_json::from_slice(bytes).expect("invalid JSON");
    build_graph(plan)
}

pub fn parse_dag_from_records(
    content: &str,
) -> (
    ExecutionGraph,
    std::collections::HashMap<String, serde_json::Value>,
) {
    let plan: crate::ExecutionPlan = serde_json::from_str(content).expect("invalid JSON");
    build_graph(plan)
}

fn build_graph(
    plan: crate::ExecutionPlan,
) -> (
    ExecutionGraph,
    std::collections::HashMap<String, serde_json::Value>,
) {
    let mut graph = ExecutionGraph::new();
    let mut payloads = std::collections::HashMap::new();
    for node in plan.nodes {
        graph.add_node(ExecutionNode {
            id: node.id.clone(),
            deps: node.deps,
        });
        payloads.insert(node.id, node.payload);
    }
    (graph, payloads)
}
