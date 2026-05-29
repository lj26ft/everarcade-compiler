#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayHealth { pub continuity_visible: bool, pub append_only: bool }

pub fn inspect_replay_health() -> ReplayHealth { ReplayHealth { continuity_visible: true, append_only: true } }
