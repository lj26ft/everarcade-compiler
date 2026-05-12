use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceOperation {
    NodeExecution,
    StateMutation,
    MemoryOperation,
    FuelAccounting,
    ContractCall,
    SnapshotLineage,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraceTransition {
    pub transition_id: String,
    pub node_id: String,
    pub operation: TraceOperation,
    pub before: String,
    pub after: String,
    pub memory_delta: i64,
    pub fuel_delta: i64,
    pub index: u64,
}
