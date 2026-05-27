use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicServiceState {
    pub continuity_root: String,
    pub replay_height: u64,
    pub operational: bool,
}

impl Default for DeterministicServiceState {
    fn default() -> Self {
        Self { continuity_root: "genesis".to_string(), replay_height: 0, operational: false }
    }
}
