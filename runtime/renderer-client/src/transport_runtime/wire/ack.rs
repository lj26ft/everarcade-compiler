use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayAckWireMessage {
    pub peer_id: String,
    pub acknowledged_sequence: u64,
    pub continuity_root: String,
    pub accepted: bool,
    pub replay_only: bool,
}
