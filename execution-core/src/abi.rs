use serde::Deserialize;

use crate::payload::Payload;

#[derive(Debug, Deserialize)]
pub struct ExecutionPlan {
    pub version: u32,
    pub nodes: Vec<ExecutionNodeAbi>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionNodeAbi {
    pub id: String,
    pub deps: Vec<String>,
    pub payload: Payload,
}

impl ExecutionPlan {
    pub fn validate(&self) {
        // 🔒 enforce ABI version
        if self.version != 1 {
            panic!("Unsupported ABI version: {}", self.version);
        }

        // 🔒 ensure unique node IDs
        let mut seen = std::collections::HashSet::new();

        for node in &self.nodes {
            if !seen.insert(&node.id) {
                panic!("Duplicate node id: {}", node.id);
            }

            node.payload.validate();
        }
    }
}
