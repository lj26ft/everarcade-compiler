use super::civilization_memory::CivilizationMemoryRecord;
#[derive(Default, Clone, Debug)]
pub struct MemoryIndex { records: Vec<CivilizationMemoryRecord> }
impl MemoryIndex {
    pub fn insert(&mut self, record: CivilizationMemoryRecord) { self.records.push(record); }
    pub fn records(&self) -> &[CivilizationMemoryRecord] { &self.records }
}
