#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayAnchorWindow { pub start_era: String, pub end_era: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayAnchorRoot { pub value: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayAnchor { pub window: HistoricalReplayAnchorWindow, pub root: HistoricalReplayAnchorRoot }
