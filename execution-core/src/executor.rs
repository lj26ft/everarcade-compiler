use std::collections::HashMap;

use crate::{
    dag::ExecutionGraph,
    hash::{hash_combine},
    state::ExecutionContext,
    types::Executable,
};

pub struct Executor {
    pub graph: ExecutionGraph,
    pub registry: HashMap<String, Box<dyn Executable>>,
}

impl Executor {
    pub fn new(graph: ExecutionGraph) -> Self {
        Self {
            graph,
            registry: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: String, exec: Box<dyn Executable>) {
        self.registry.insert(id, exec);
    }

    pub fn execute(&mut self) -> ExecutionContext {
        let mut ctx = ExecutionContext::new();

        let order = self.graph.topo_sort();

        for node_id in order {
            let exec = self
                .registry
                .get(&node_id)
                .expect("missing executable");

            let output = exec.execute(&mut ctx);

            // gather dependency hashes
            let deps = &self.graph.nodes.get(&node_id).unwrap().deps;

            let mut dep_hash_bytes: Vec<Vec<u8>> = Vec::new();

            for dep in deps {
                let h = ctx.get_hash(dep).expect("missing dep hash");
                dep_hash_bytes.push(h.to_vec());
            }

            // compute node hash
            let node_hash = hash_combine(&[
                node_id.as_bytes(),
                &output,
                &dep_hash_bytes.concat(),
            ]);

            ctx.set(node_id.clone(), output, node_hash);
        }

        ctx
    }
}
