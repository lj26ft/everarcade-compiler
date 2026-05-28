#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayReplayCheckpoint {
    pub tick: u64,
    pub continuity_root: String,
    pub state_root: String,
}
