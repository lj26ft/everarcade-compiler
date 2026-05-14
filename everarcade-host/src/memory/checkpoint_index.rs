use super::civilization_memory::{CivilizationMemoryRecord, Hash};
pub fn checkpoint_lookup(records: &[CivilizationMemoryRecord], checkpoint_root: Hash) -> Option<CivilizationMemoryRecord> {
    records.iter().copied().find(|r| r.checkpoint_root == checkpoint_root)
}
