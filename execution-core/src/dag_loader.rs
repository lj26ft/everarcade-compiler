use std::fs;

use crate::{
    abi::ExecutionPlan,
    dag::{ExecutionGraph, ExecutionNode},
    payload::Payload,
};

pub fn load_graph_from_file(
    path: &str,
) -> (ExecutionGraph, std::collections::HashMap<String, Payload>) {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("failed to read file: {}", path));

    load_graph_from_str(&content)
}

pub fn load_graph_from_str(
    content: &str,
) -> (ExecutionGraph, std::collections::HashMap<String, Payload>) {
    let plan: ExecutionPlan =
        serde_json::from_str(content).expect("invalid JSON");

    plan.validate();

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
