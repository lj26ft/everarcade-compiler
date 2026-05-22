use execution_core::client_protocol::{
    AssetReference, EntitySnapshot, PlayerInputCommand, SimulationFrame, WorldSnapshot,
};

pub fn get_world_snapshot() -> WorldSnapshot {
    WorldSnapshot {
        tick: 0,
        world_id: "runtime/world".into(),
        entities: vec![],
    }
}

pub fn get_entity_snapshot(entity_id: &str) -> EntitySnapshot {
    EntitySnapshot {
        entity_id: entity_id.into(),
        entity_type: "unknown".into(),
        state: serde_json::json!({}),
    }
}

pub fn submit_player_command(
    player_id: &str,
    command: &str,
    payload: serde_json::Value,
) -> PlayerInputCommand {
    PlayerInputCommand {
        player_id: player_id.into(),
        command: command.into(),
        payload,
    }
}

pub fn subscribe_simulation_events() -> Vec<SimulationFrame> {
    vec![]
}

pub fn load_asset_manifest() -> Vec<AssetReference> {
    vec![]
}
