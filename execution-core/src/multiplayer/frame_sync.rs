#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameSync {
    pub frame: u64,
    pub ordered_players: Vec<String>,
    pub continuity_root: String,
}
