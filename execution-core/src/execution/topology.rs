use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub fn canonical_topological_sort(
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> Result<Vec<String>, String> {
    let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
    for (from, tos) in edges {
        indegree.entry(from.clone()).or_insert(0);
        for to in tos {
            *indegree.entry(to.clone()).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<String> = indegree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(n, _)| n.clone())
        .collect();

    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        if let Some(children) = edges.get(&node) {
            for child in children {
                if let Some(d) = indegree.get_mut(child) {
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(child.clone());
                    }
                }
            }
        }
    }

    if order.len() != indegree.len() {
        return Err("execution graph contains cycle".to_string());
    }
    Ok(order)
}

pub fn detect_execution_cycles(edges: &BTreeMap<String, BTreeSet<String>>) -> bool {
    canonical_topological_sort(edges).is_err()
}

pub fn canonical_execution_batches(
    order: &[String],
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<Vec<String>> {
    let mut level: BTreeMap<String, usize> = BTreeMap::new();
    for node in order {
        let mut max_parent = 0usize;
        for (parent, children) in edges {
            if children.contains(node) {
                max_parent = max_parent.max(level.get(parent).copied().unwrap_or(0) + 1);
            }
        }
        level.insert(node.clone(), max_parent);
    }

    let mut grouped: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for (node, lv) in level {
        grouped.entry(lv).or_default().push(node);
    }
    grouped.into_values().collect()
}
