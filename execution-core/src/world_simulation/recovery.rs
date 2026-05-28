use super::runtime::WorldSimulationRuntime;
pub fn restore_world_simulation(snapshot: &WorldSimulationRuntime) -> WorldSimulationRuntime {
    snapshot.clone()
}
