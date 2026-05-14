use super::civilization_memory::{CivilizationMemoryRecord, Hash};
pub fn continuity_reconstruct(records: &[CivilizationMemoryRecord], continuity_root: Hash) -> Vec<CivilizationMemoryRecord> {
    records.iter().copied().filter(|r| r.continuity_root == continuity_root).collect()
}
