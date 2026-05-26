use super::index::HistoricalReplayIndex;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayQuery {
    pub key: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayResult {
    pub key: String,
    pub era_id: String,
    pub frame: u64,
    pub provenance_root: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayQueryCursor {
    pub offset: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayQueryWindow {
    pub era_id: String,
    pub start_frame: u64,
    pub end_frame: u64,
}
#[derive(Debug, Default)]
pub struct HistoricalReplayQueryExecutor;
#[derive(Debug, Default)]
pub struct HistoricalReplayQueryRuntime;
impl HistoricalReplayQueryRuntime {
    pub fn query(
        indexes: &[HistoricalReplayIndex],
        query: &HistoricalReplayQuery,
    ) -> Option<HistoricalReplayResult> {
        indexes
            .iter()
            .find(|i| i.key == query.key)
            .map(|i| HistoricalReplayResult {
                key: i.key.clone(),
                era_id: i.era_id.clone(),
                frame: i.frame,
                provenance_root: i.provenance_root.clone(),
            })
    }
}
impl HistoricalReplayQueryExecutor {
    pub fn window(
        indexes: &[HistoricalReplayIndex],
        window: &HistoricalReplayQueryWindow,
        cursor: &HistoricalReplayQueryCursor,
    ) -> Vec<HistoricalReplayResult> {
        indexes
            .iter()
            .skip(cursor.offset)
            .filter(|i| {
                i.era_id == window.era_id
                    && i.frame >= window.start_frame
                    && i.frame <= window.end_frame
            })
            .map(|i| HistoricalReplayResult {
                key: i.key.clone(),
                era_id: i.era_id.clone(),
                frame: i.frame,
                provenance_root: i.provenance_root.clone(),
            })
            .collect()
    }
}
