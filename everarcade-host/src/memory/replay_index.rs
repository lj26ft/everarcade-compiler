use super::civilization_memory::{CivilizationMemoryRecord, Hash};
pub fn replay_lookup(
    records: &[CivilizationMemoryRecord],
    replay_root: Hash,
) -> Option<CivilizationMemoryRecord> {
    records
        .iter()
        .copied()
        .find(|r| r.replay_root == replay_root)
}
