use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldTick(pub u64);

pub fn compute_tick_hash(
    tick_id: u64,
    timeline_hash: [u8; 32],
    event_hash: [u8; 32],
    state_root: [u8; 32],
    entity_continuity_hash: [u8; 32],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(tick_id.to_le_bytes());
    hasher.update(timeline_hash);
    hasher.update(event_hash);
    hasher.update(state_root);
    hasher.update(entity_continuity_hash);
    hasher.finalize().into()
}

pub fn verify_tick_order(expected_next: u64, candidate: u64) -> bool {
    expected_next == candidate
}
