// FILE: execution-core/src/scheduler.rs
//
// DETERMINISTIC TOPOLOGICAL SORT
//
// RULE:
// ONLY BTreeMap / BTreeSet
//

use crate::ExecutionNode;

use std::collections::{BTreeMap, BTreeSet};

pub fn topological_sort(nodes: &[ExecutionNode]) -> Vec<ExecutionNode> {
    let mut node_map = BTreeMap::<String, ExecutionNode>::new();

    for node in nodes {
        node_map.insert(node.id.clone(), node.clone());
    }

    //
    // VALIDATE DEPENDENCIES
    //

    for node in nodes {
        for dep in &node.deps {
            if !node_map.contains_key(dep) {
                panic!("missing dependency: {}", dep);
            }
        }
    }

    //
    // DFS
    //

    let mut ordered = Vec::<ExecutionNode>::new();

    let mut temporary = BTreeSet::<String>::new();

    let mut permanent = BTreeSet::<String>::new();

    let keys: Vec<String> = node_map.keys().cloned().collect();

    for key in keys {
        visit(
            &key,
            &node_map,
            &mut temporary,
            &mut permanent,
            &mut ordered,
        );
    }

    ordered
}

fn visit(
    id: &str,
    node_map: &BTreeMap<String, ExecutionNode>,
    temporary: &mut BTreeSet<String>,
    permanent: &mut BTreeSet<String>,
    ordered: &mut Vec<ExecutionNode>,
) {
    if permanent.contains(id) {
        return;
    }

    if temporary.contains(id) {
        panic!("cycle detected");
    }

    temporary.insert(id.to_string());

    let node = node_map.get(id).expect("missing node");

    for dep in &node.deps {
        visit(dep, node_map, temporary, permanent, ordered);
    }

    temporary.remove(id);

    permanent.insert(id.to_string());

    ordered.push(node.clone());
}
