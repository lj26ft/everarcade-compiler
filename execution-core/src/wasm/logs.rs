use super::events::ExecutionEvent;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicEventLog {
    pub events: Vec<ExecutionEvent>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionLogEnvelope {
    pub log: DeterministicEventLog,
}
