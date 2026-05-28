use crate::gameplay::replay::GameplayReplayCheckpoint;
pub fn restore_checkpoint(
    tick: u64,
    continuity_root: impl Into<String>,
    state_root: impl Into<String>,
) -> GameplayReplayCheckpoint {
    GameplayReplayCheckpoint {
        tick,
        continuity_root: continuity_root.into(),
        state_root: state_root.into(),
    }
}
