pub type Hash = [u8; 32];
use super::civilization_memory::CivilizationMemoryRecord;
use sha2::{Digest, Sha256};
pub fn memory_root(records: &[CivilizationMemoryRecord]) -> Hash {
    let mut h = Sha256::new();
    for r in records {
        h.update(r.civilization_root);
        h.update(r.replay_root);
        h.update(r.checkpoint_root);
        h.update(r.continuity_root);
        h.update(r.epoch_index.to_le_bytes());
    }
    h.finalize().into()
}
