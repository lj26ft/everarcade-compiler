use crate::simulation::{behavior::EntityBehavior, state::SimulationState};

pub fn execute_entity_behavior(state: &mut SimulationState, behavior: &EntityBehavior) {
    let delta = match behavior.opcode.as_str() {
        "idle" => 0,
        "move" => 1,
        "attack" => 2,
        _ => 1,
    } * behavior.intensity;
    *state
        .entities
        .entry(behavior.entity_id.clone())
        .or_insert(0) += delta;
}

pub fn verify_entity_simulation(expected: &SimulationState, actual: &SimulationState) -> bool {
    expected.entities == actual.entities
}

pub fn replay_entity_state(state: &mut SimulationState, snapshot: &SimulationState) {
    state.entities = snapshot.entities.clone();
}
