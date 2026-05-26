use super::{
    authority::authoritative,
    input::PlayerInput,
    input_runtime::{canonicalize_inputs, InputAction, RuntimeInput},
    inventory::InventoryState,
    physics::apply_input,
    tick::{advance_tick, window_roots},
    world_state::WorldState,
};
use crate::hashing::hash_bytes;

pub fn step(mut world: WorldState, inputs: &[PlayerInput]) -> WorldState {
    for input in inputs {
        for entity in world.entities.values_mut() {
            if authoritative(entity, &input.player_id) {
                apply_input(entity, input);
            }
        }
    }
    world.tick = advance_tick(world.tick);
    world
}

#[derive(Debug, Clone)]
pub struct SimulationOutput {
    pub world: WorldState,
    pub inventory: InventoryState,
    pub state_root: String,
    pub event_root: String,
    pub validation_root: String,
}

pub fn step_runtime(
    mut world: WorldState,
    mut inputs: Vec<RuntimeInput>,
    mut inventory: InventoryState,
) -> SimulationOutput {
    canonicalize_inputs(&mut inputs);
    let bounds = (-10, 10, -10, 10);
    let mut events = Vec::new();
    for input in &inputs {
        for entity in world.entities.values_mut() {
            if entity.authority != input.player_id {
                continue;
            }
            match input.action {
                InputAction::MoveUp => entity.apply_bounded_movement(0, 1, bounds),
                InputAction::MoveDown => entity.apply_bounded_movement(0, -1, bounds),
                InputAction::MoveLeft => entity.apply_bounded_movement(-1, 0, bounds),
                InputAction::MoveRight => entity.apply_bounded_movement(1, 0, bounds),
                InputAction::Interact => events.push(format!("interact:{}", input.player_id)),
                InputAction::InventoryAction => {
                    let item = format!("demo-item-{}", world.tick);
                    inventory.add_item(&input.player_id, &item);
                    events.push(format!("inventory:{}:{}", input.player_id, item));
                }
            }
        }
    }
    world.tick = advance_tick(world.tick);
    let state_root = hash_bytes(serde_json::to_string(&world).unwrap().as_bytes());
    let event_root = hash_bytes(serde_json::to_string(&events).unwrap().as_bytes());
    let (_, _, validation_root) = window_roots(world.tick, &state_root, &event_root);
    SimulationOutput {
        world,
        inventory,
        state_root,
        event_root,
        validation_root,
    }
}
