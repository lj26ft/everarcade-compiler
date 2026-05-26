use super::index::HistoricalReplayIndex;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayQuery { pub key: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayResult { pub key: String, pub era_id: String, pub frame: u64, pub provenance_root: String }
#[derive(Debug, Default)]
pub struct HistoricalReplayQueryRuntime;
impl HistoricalReplayQueryRuntime { pub fn query(indexes: &[HistoricalReplayIndex], query: &HistoricalReplayQuery) -> Option<HistoricalReplayResult> { indexes.iter().find(|i| i.key == query.key).map(|i| HistoricalReplayResult { key: i.key.clone(), era_id: i.era_id.clone(), frame: i.frame, provenance_root: i.provenance_root.clone() }) } }
