use crate::trace::transition::TraceTransition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraceNode {
    pub node_id: String,
    pub op_code: String,
    pub input_hash: String,
    pub output_hash: String,
    pub index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionTrace {
    pub trace_id: String,
    pub epoch_id: u64,
    pub execution_root: String,
    pub state_root_before: String,
    pub state_root_after: String,
    pub nodes: Vec<TraceNode>,
    pub transitions: Vec<TraceTransition>,
    pub fuel_used: u64,
    pub memory_used: u64,
}
