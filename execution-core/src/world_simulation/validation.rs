use super::runtime::WorldSimulationRuntime;
pub fn world_is_deterministic(world: &WorldSimulationRuntime) -> bool {
    world
        .terrain
        .windows(2)
        .all(|w| (w[0].x, w[0].y) <= (w[1].x, w[1].y))
        && world.replay_roots.iter().all(|r| !r.is_empty())
}
pub fn world_equivalent(a: &WorldSimulationRuntime, b: &WorldSimulationRuntime) -> bool {
    a == b
}
