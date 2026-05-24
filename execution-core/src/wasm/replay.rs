use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayValidationReceipt {
    pub equivalent: bool,
}
pub struct ExecutionReplayEngine;
