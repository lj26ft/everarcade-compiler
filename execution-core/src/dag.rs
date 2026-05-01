use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct ExecutionNode {
    pub id: String,
    pub deps: Vec<String>,
}

pub struct ExecutionGraph {
    pub nodes: HashMap<String, ExecutionNode>,
}

impl ExecutionGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: ExecutionNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn topo_sort(&self) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut order = Vec::new();

        fn visit(
            id: &String,
            graph: &ExecutionGraph,
            visited: &mut HashSet<String>,
            order: &mut Vec<String>,
        ) {
            if visited.contains(id) {
                return;
            }

            let node = graph.nodes.get(id).expect("missing node");

            for dep in &node.deps {
                visit(dep, graph, visited, order);
            }

            visited.insert(id.clone());
            order.push(id.clone());
        }

        for id in self.nodes.keys() {
            visit(id, self, &mut visited, &mut order);
        }

        order
    }
}
