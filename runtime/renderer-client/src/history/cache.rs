#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayCacheWindow {
    pub start_frame: u64,
    pub end_frame: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayCacheManifest {
    pub continuity_root: String,
    pub windows: Vec<HistoricalReplayCacheWindow>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayCache {
    pub manifest: HistoricalReplayCacheManifest,
}
