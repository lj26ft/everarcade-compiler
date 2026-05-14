use super::{civilization_memory::CivilizationMemoryRecord, memory_root::memory_root};
pub fn validate_memory_root(records: &[CivilizationMemoryRecord], expected: [u8; 32]) -> bool {
    memory_root(records) == expected
}
