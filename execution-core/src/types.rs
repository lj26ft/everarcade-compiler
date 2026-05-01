use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

pub type State = BTreeMap<String, String>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VmInput {
    pub state: State,
    pub plan: ExecutionPlan,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VmOutput {
    pub updated_state: State,
    pub receipt: ExecutionReceipt,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionPlan {
    pub nodes: Vec<ExecutionNode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionNode {
    pub contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionReceipt {
    pub state_root_before: String,
    pub state_root_after: String,
    pub execution_root: String,
    pub receipt_hash: String,
    pub node_hashes: Vec<String>,
    pub state_changes: Vec<StateChange>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StateChange {
    pub key: String,
    pub before: String,
    pub after: String,
}
