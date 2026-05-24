use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoredExecutionState {
    pub state_root: String,
}
pub struct ExecutionRestoreEngine;
