use crate::gameplay::replay::GameplayReplayContinuity;
pub fn restore_replay_continuity(
    root: impl Into<String>,
    latest_tick: u64,
) -> GameplayReplayContinuity {
    GameplayReplayContinuity {
        continuity_root: root.into(),
        latest_tick,
        append_only: true,
    }
}
