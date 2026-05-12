use std::collections::{BTreeMap, BTreeSet};
use execution_core::execution::{execute_graph, ExecutionState};
use execution_core::federation::execution_graph::ExecutionGraph;

#[test]
fn deterministic_scheduler_order() {
    let mut edges = BTreeMap::new();
    edges.insert("a".to_string(), BTreeSet::from(["b".to_string()]));
    edges.insert("b".to_string(), BTreeSet::new());
    let out = execute_graph(ExecutionGraph { edges }, ExecutionState::default());
    assert_eq!(out.stable_receipt_order, vec!["a", "b"]);
}
