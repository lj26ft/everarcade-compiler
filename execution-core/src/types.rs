use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionNode {
    pub id: String,
    pub action: String,
    pub payload: Value,
    pub deps: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionPlan {
    pub nodes: Vec<ExecutionNode>,
}
