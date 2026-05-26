use super::input::PlayerInput;
use serde::{Deserialize, Serialize};

pub fn deterministic_order(mut inputs: Vec<PlayerInput>) -> Vec<PlayerInput> {
    inputs.sort_by(|a, b| a.player_id.cmp(&b.player_id));
    inputs
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSessionEnvelope {
    pub session_id: String,
    pub player_id: String,
    pub authoritative_tick: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteProjectionFrame {
    pub tick: u64,
    pub projection_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiplayerProjectionAnchor {
    pub local_root: String,
    pub remote_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractiveRuntimeWitness {
    pub validation_root: String,
    pub checkpoint_root: String,
}
