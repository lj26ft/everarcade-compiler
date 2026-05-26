use super::{authority::authoritative, input::PlayerInput, physics::apply_input, tick::advance_tick, world_state::WorldState};
pub fn step(mut world: WorldState, inputs: &[PlayerInput]) -> WorldState { for input in inputs { for entity in world.entities.values_mut() { if authoritative(entity, &input.player_id) { apply_input(entity, input); } } } world.tick = advance_tick(world.tick); world }
