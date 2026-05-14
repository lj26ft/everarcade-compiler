use super::civilization_memory::CivilizationMemoryRecord;
pub fn query_by_epoch(
    records: &[CivilizationMemoryRecord],
    epoch_index: u64,
) -> Option<CivilizationMemoryRecord> {
    records
        .iter()
        .copied()
        .find(|r| r.epoch_index == epoch_index)
}
