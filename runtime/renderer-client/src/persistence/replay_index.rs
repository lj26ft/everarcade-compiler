#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LiveReplayIndex {
    pub replay_tip: u64,
    pub continuity_root: String,
    pub append_only: bool,
}
impl LiveReplayIndex {
    pub fn restore(replay_tip: u64, continuity_root: impl Into<String>) -> Result<Self, String> {
        let continuity_root = continuity_root.into();
        if continuity_root.is_empty() {
            return Err("corrupted_replay_history_rejected".into());
        }
        Ok(Self {
            replay_tip,
            continuity_root,
            append_only: true,
        })
    }
}
