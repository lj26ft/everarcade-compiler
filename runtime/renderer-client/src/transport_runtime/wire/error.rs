use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayTransportError {
    pub code: String,
    pub message: String,
    pub replay_only: bool,
}

impl ReplayTransportError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            replay_only: true,
        }
    }
}
