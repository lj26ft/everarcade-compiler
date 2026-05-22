use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInputCommand {
    pub player_id: String,
    pub command: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: u64,
    pub world_id: String,
    pub entities: Vec<EntitySnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub entity_id: String,
    pub entity_type: String,
    pub state: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFrame {
    pub tick: u64,
    pub interactions: Vec<InteractionEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub event_type: String,
    pub entity_id: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFrame {
    pub frame_id: String,
    pub tick: u64,
    pub world_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    pub asset_id: String,
    pub asset_type: String,
    pub content_hash: String,
    pub path: String,
    pub version: String,
}
