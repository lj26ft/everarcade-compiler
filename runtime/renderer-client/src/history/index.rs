#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayIndex { pub key: String, pub era_id: String, pub frame: u64, pub provenance_root: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayIndexSegment { pub era_id: String, pub entries: Vec<HistoricalReplayIndex> }
#[derive(Debug, Default)]
pub struct HistoricalReplayIndexWriter;
#[derive(Debug, Default)]
pub struct HistoricalReplayIndexReader;
#[derive(Debug, Default)]
pub struct HistoricalIndexRuntime;
impl HistoricalReplayIndexWriter { pub fn write(segment: &HistoricalReplayIndexSegment) -> Vec<HistoricalReplayIndex> { let mut entries = segment.entries.clone(); entries.sort_by(|a,b| a.key.cmp(&b.key)); entries } }
impl HistoricalReplayIndexReader { pub fn lookup<'a>(entries: &'a [HistoricalReplayIndex], key: &str) -> Option<&'a HistoricalReplayIndex> { entries.iter().find(|e| e.key == key) } }
impl HistoricalIndexRuntime { pub fn seek(entries: &[HistoricalReplayIndex], era_id: &str, frame: u64) -> Option<HistoricalReplayIndex> { entries.iter().find(|e| e.era_id == era_id && e.frame == frame).cloned() } }
