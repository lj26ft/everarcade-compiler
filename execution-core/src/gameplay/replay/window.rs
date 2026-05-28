#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayReplayWindow {
    pub start_tick: u64,
    pub end_tick: u64,
    pub continuity_root: String,
    pub state_root: String,
    pub manifest_root: String,
}
