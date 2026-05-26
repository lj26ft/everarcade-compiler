use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnchorCommitment {
    pub world_id: String,
    pub state_root: String,
    pub replay_root: String,
    pub tick: u64,
}
