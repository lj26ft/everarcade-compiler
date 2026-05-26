use serde::{Deserialize, Serialize};

use super::input_runtime::RuntimeInput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayTickRecord {
    pub tick: u64,
    pub inputs: Vec<RuntimeInput>,
    pub state_root: String,
    pub event_root: String,
    pub validation_root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRecord {
    pub ticks: Vec<ReplayTickRecord>,
}
