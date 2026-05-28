use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiDecision {
    pub entity_id: String,
    pub tick: u64,
    pub action: String,
    pub replay_root: String,
}
