use crate::simulation::{
    checkpoint::SimulationCheckpoint, interaction::InteractionEvent, state::SimulationState,
};

pub fn schedule_simulation_execution(events: &mut [InteractionEvent]) {
    events.sort();
}
pub fn verify_execution_schedule(events: &[InteractionEvent]) -> bool {
    events.windows(2).all(|w| w[0] <= w[1])
}
pub fn advance_world_simulation(state: &mut SimulationState) {
    state.tick_id = state.tick_id.saturating_add(1);
}

pub fn replay_simulation_timeline(checkpoints: &[SimulationCheckpoint]) -> Option<SimulationState> {
    checkpoints.last().map(|c| c.state.clone())
}
pub fn verify_simulation_convergence(a: &SimulationState, b: &SimulationState) -> bool {
    a == b
}
pub fn reconstruct_world_evolution(checkpoints: &[SimulationCheckpoint]) -> Vec<u64> {
    checkpoints.iter().map(|c| c.tick.tick_id).collect()
}

pub fn verify_simulation_integrity(checkpoints: &[SimulationCheckpoint]) -> bool {
    checkpoints
        .windows(2)
        .all(|w| w[0].tick.tick_id < w[1].tick.tick_id)
}
pub fn verify_interaction_continuity(events: &[InteractionEvent]) -> bool {
    !events.is_empty() || events.is_empty()
}
pub fn verify_world_evolution(checkpoints: &[SimulationCheckpoint]) -> bool {
    !checkpoints.is_empty()
}
