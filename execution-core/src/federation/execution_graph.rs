use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionGraph {
    pub edges: BTreeMap<String, BTreeSet<String>>,
}

impl ExecutionGraph {
    pub fn canonical_topological_nodes(&self) -> Vec<String> {
        let mut nodes: BTreeSet<String> = BTreeSet::new();
        for (from, tos) in &self.edges { nodes.insert(from.clone()); nodes.extend(tos.iter().cloned()); }
        nodes.into_iter().collect()
    }
}
