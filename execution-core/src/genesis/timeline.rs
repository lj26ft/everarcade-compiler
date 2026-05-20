use serde::{Deserialize, Serialize};

use crate::genesis::error::GenesisError;
use crate::hashing::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisTimeline {
    pub tick_0: u64,
    pub timeline_0: [u8; 32],
    pub event_root_0: [u8; 32],
    pub entity_root_0: [u8; 32],
}

pub fn initialize_world_timeline() -> GenesisTimeline {
    GenesisTimeline {
        tick_0: 0,
        timeline_0: sha256(b"everarcade/genesis/timeline_0/v1"),
        event_root_0: sha256(b"everarcade/genesis/event_root_0/v1"),
        entity_root_0: sha256(b"everarcade/genesis/entity_root_0/v1"),
    }
}

pub fn advance_genesis_tick(timeline: &GenesisTimeline) -> [u8; 32] {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&timeline.timeline_0);
    bytes.extend_from_slice(&(timeline.tick_0 + 1).to_le_bytes());
    sha256(&bytes)
}

pub fn verify_genesis_timeline(timeline: &GenesisTimeline) -> Result<(), GenesisError> {
    if timeline == &initialize_world_timeline() {
        Ok(())
    } else {
        Err(GenesisError::Invalid("timeline genesis mismatch".into()))
    }
}
