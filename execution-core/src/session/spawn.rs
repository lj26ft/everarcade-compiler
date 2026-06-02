use super::player::Character;
use contract_api::protocol_records::ProtocolRecord;
use rustrigs::movement::{move_actor, MovementInput};

pub fn spawn_player(character: &mut Character, tick: u64) -> Vec<ProtocolRecord> {
    character.x = 0;
    character.y = 0;
    character.alive = true;
    move_actor(MovementInput {
        actor: character.id.0.clone(),
        world: "Arena Outpost".to_owned(),
        x: 0,
        y: 0,
        min: -100,
        max: 100,
        tick,
    })
}

pub fn despawn_player(character: &mut Character, tick: u64) -> Vec<ProtocolRecord> {
    character.alive = false;
    move_actor(MovementInput {
        actor: character.id.0.clone(),
        world: "Arena Outpost".to_owned(),
        x: character.x,
        y: character.y,
        min: -100,
        max: 100,
        tick,
    })
}

pub fn respawn_player(character: &mut Character, tick: u64) -> Vec<ProtocolRecord> {
    character.health = character.max_health;
    character.energy = character.max_energy;
    spawn_player(character, tick)
}
