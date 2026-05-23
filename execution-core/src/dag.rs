use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub struct ExecutionNode {
    pub id: String,
    pub deps: Vec<String>,
}

pub struct ExecutionGraph {
    pub nodes: BTreeMap<String, ExecutionNode>,
}

impl ExecutionGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: ExecutionNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn topo_sort(&self) -> Vec<String> {
        self.topo_sort_checked().unwrap_or_default()
    }

    pub fn topo_sort_checked(&self) -> anyhow::Result<Vec<String>> {
        let mut done = BTreeSet::new();
        let mut ordered = Vec::new();
        loop {
            let mut progressed = false;
            for (id, node) in &self.nodes {
                if done.contains(id) {
                    continue;
                }
                if node.deps.iter().all(|d| done.contains(d)) {
                    done.insert(id.clone());
                    ordered.push(id.clone());
                    progressed = true;
                }
            }
            if !progressed {
                break;
            }
        }
        if done.len() != self.nodes.len() {
            anyhow::bail!("cycle detected")
        }
        Ok(ordered)
    }
}
