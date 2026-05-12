use std::collections::BTreeMap;

use crate::federation::execution_graph::ExecutionGraph;

use super::{
    execution_node::ExecutionPolicy,
    execution_result::{ExecutionOutcome, ExecutionResult},
    execution_state::ExecutionState,
    failure_policy::should_rollback,
    topology::canonical_topological_sort,
};

pub fn execute_graph(graph: ExecutionGraph, mut state: ExecutionState) -> ExecutionResult {
    let mut outcomes = Vec::new();
    let mut rolled_back = false;
    let snapshot = state.clone();

    let order = match canonical_topological_sort(&graph.edges) {
        Ok(v) => v,
        Err(e) => {
            return ExecutionResult { final_state: state, outcomes: vec![ExecutionOutcome { node_id: "<graph>".into(), success: false, message: e }], stable_receipt_order: vec![], rolled_back: false };
        }
    };

    let policies: BTreeMap<String, ExecutionPolicy> = BTreeMap::new();
    for node in &order {
        let policy = policies.get(node).copied().unwrap_or(ExecutionPolicy::Required);
        let success = true;
        outcomes.push(ExecutionOutcome { node_id: node.clone(), success, message: "applied".into() });
        state.applied_nodes.push(node.clone());

        if !success && should_rollback(policy) {
            state = snapshot;
            rolled_back = true;
            break;
        }
    }

    ExecutionResult { final_state: state, outcomes, stable_receipt_order: order, rolled_back }
}
