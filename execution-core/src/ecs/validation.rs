use super::{
    runtime::{EcsReplayWindow, EcsRuntime},
    system::DeterministicSystem,
};

pub fn systems_are_canonical(systems: &[DeterministicSystem]) -> bool {
    systems.windows(2).all(|w| w[0].id <= w[1].id)
}

pub fn replay_equivalent(a: &EcsRuntime, b: &EcsRuntime) -> bool {
    a.storage == b.storage && a.replay == b.replay
}

pub fn replay_window_is_append_only(window: &EcsReplayWindow) -> bool {
    window.events.windows(2).all(|w| w[0].tick <= w[1].tick)
}
