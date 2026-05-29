#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreservedState { pub tick: u64, pub replay_continuity: bool }

pub fn preserve_state(tick: u64) -> PreservedState { PreservedState { tick, replay_continuity: true } }
