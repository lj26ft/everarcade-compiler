use serde::{Deserialize, Serialize};

use crate::genesis::error::GenesisError;
use crate::genesis::genesis::GenesisState;
use crate::hashing::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisCheckpoint {
    pub world_root: [u8; 32],
    pub timeline_root: [u8; 32],
    pub continuity_root: [u8; 32],
    pub protocol_version: u32,
    pub topology_epoch: u64,
    pub checkpoint_root: [u8; 32],
}

pub fn create_genesis_checkpoint(state: &GenesisState) -> GenesisCheckpoint {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&state.world_root.world_root);
    bytes.extend_from_slice(&state.timeline.timeline_0);
    bytes.extend_from_slice(&state.continuity.federation_continuity_root);
    bytes.extend_from_slice(&state.protocol_version.to_le_bytes());
    bytes.extend_from_slice(&state.continuity.topology_epoch.to_le_bytes());
    let checkpoint_root = sha256(&bytes);
    GenesisCheckpoint {
        world_root: state.world_root.world_root,
        timeline_root: state.timeline.timeline_0,
        continuity_root: state.continuity.federation_continuity_root,
        protocol_version: state.protocol_version,
        topology_epoch: state.continuity.topology_epoch,
        checkpoint_root,
    }
}

pub fn verify_genesis_checkpoint(checkpoint: &GenesisCheckpoint) -> Result<(), GenesisError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&checkpoint.world_root);
    bytes.extend_from_slice(&checkpoint.timeline_root);
    bytes.extend_from_slice(&checkpoint.continuity_root);
    bytes.extend_from_slice(&checkpoint.protocol_version.to_le_bytes());
    bytes.extend_from_slice(&checkpoint.topology_epoch.to_le_bytes());
    if sha256(&bytes) == checkpoint.checkpoint_root {
        Ok(())
    } else {
        Err(GenesisError::Invalid("checkpoint root mismatch".into()))
    }
}
