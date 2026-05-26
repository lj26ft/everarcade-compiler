use super::storage::{HistoricalArtifactRecord, HistoricalArtifactStore};

#[derive(Debug, Clone)]
pub struct HistoricalArtifactIoRuntime { pub store: HistoricalArtifactStore }
impl HistoricalArtifactIoRuntime {
    pub fn persist(&self, record: &HistoricalArtifactRecord) -> Result<(), String> { self.store.writer().append(record) }
    pub fn restore(&self) -> Result<Vec<HistoricalArtifactRecord>, String> { self.store.reader().read_all() }
}
