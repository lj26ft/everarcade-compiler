use serde::{Deserialize, Serialize};
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
