#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayFederationWindow {
    pub window_id: String,
    pub start: u64,
    pub end: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayWindowManifest {
    pub window_id: String,
    pub continuity_root: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayWindowContinuity {
    pub previous_root: String,
    pub root: String,
}
