use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectionFederationWindow {
    pub session_id: String,
    pub window_id: String,
    pub start_frame: u64,
    pub end_frame: u64,
    pub continuity_root: String,
    pub artifact_root: String,
}

impl ProjectionFederationWindow {
    pub fn is_valid(&self) -> bool { self.start_frame <= self.end_frame }
}
