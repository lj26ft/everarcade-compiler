use sha2::{Digest, Sha256};

use crate::simulation::{
    error::SimulationError, interaction::InteractionEvent, state::SimulationState,
    tick::SimulationTick,
};

pub fn compute_simulation_hash(state: &SimulationState, events: &[InteractionEvent]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(serde_json::to_vec(state).unwrap_or_default());
    h.update(serde_json::to_vec(events).unwrap_or_default());
    h.finalize().into()
}

pub fn advance_simulation_tick(
    state: &mut SimulationState,
    expected_tick_id: u64,
    events: &[InteractionEvent],
) -> Result<SimulationTick, SimulationError> {
    if state.tick_id + 1 != expected_tick_id {
        return Err(SimulationError::OutOfOrderTick {
            expected: state.tick_id + 1,
            got: expected_tick_id,
        });
    }
    state.tick_id = expected_tick_id;
    let sim = compute_simulation_hash(state, events);
    Ok(SimulationTick {
        tick_id: expected_tick_id,
        simulation_hash: sim,
        interaction_hash: sim,
        entity_state_hash: sim,
        world_state_root: sim,
    })
}

pub fn verify_simulation_tick(
    state: &SimulationState,
    tick: &SimulationTick,
    events: &[InteractionEvent],
) -> Result<(), SimulationError> {
    let hash = compute_simulation_hash(state, events);
    if hash != tick.simulation_hash {
        return Err(SimulationError::HashMismatch);
    }
    Ok(())
}
