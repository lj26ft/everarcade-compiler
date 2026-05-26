use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionReplayCheckpoint {
    pub checkpoint_id: String,
    pub session_id: String,
    pub frame_index: u64,
    pub continuity_root: String,
    pub artifact_root: String,
}
