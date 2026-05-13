use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QueueState {
    Pending,
    Processing,
    Complete,
    Failed,
    Retryable,
}
