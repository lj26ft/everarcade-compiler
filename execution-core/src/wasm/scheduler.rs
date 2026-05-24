use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExecutionNodeId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionDependency {
    pub node: ExecutionNodeId,
    pub depends_on: ExecutionNodeId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicExecutionOrder {
    pub nodes: Vec<ExecutionNodeId>,
    pub dependencies: Vec<ExecutionDependency>,
}

impl DeterministicExecutionOrder {
    pub fn canonicalize(&mut self) {
        self.nodes.sort();
        self.dependencies.sort_by(|a, b| {
            (a.node.0.as_str(), a.depends_on.0.as_str())
                .cmp(&(b.node.0.as_str(), b.depends_on.0.as_str()))
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionTick {
    pub height: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledExecutionEnvelope {
    pub tick: ExecutionTick,
    pub contract_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicExecutionQueue {
    pub items: Vec<ScheduledExecutionEnvelope>,
}
pub struct ExecutionScheduler;
