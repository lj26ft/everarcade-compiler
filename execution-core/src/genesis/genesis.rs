use serde::{Deserialize, Serialize};

use crate::genesis::checkpoint::{
    create_genesis_checkpoint, verify_genesis_checkpoint, GenesisCheckpoint,
};
use crate::genesis::continuity::{
    initialize_federation_genesis, verify_federation_genesis, GenesisContinuity,
};
use crate::genesis::entity::{
    initialize_entity_lineage, verify_entity_genesis, GenesisEntityLineage,
};
use crate::genesis::error::GenesisError;
use crate::genesis::timeline::{
    initialize_world_timeline, verify_genesis_timeline, GenesisTimeline,
};
use crate::genesis::world::GenesisWorldRoot;
use crate::hashing::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisState {
    pub protocol_version: u32,
    pub world_root: GenesisWorldRoot,
    pub checkpoint_root: [u8; 32],
    pub timeline_root: [u8; 32],
    pub entity_root: [u8; 32],
    pub continuity_root: [u8; 32],
    pub timeline: GenesisTimeline,
    pub continuity: GenesisContinuity,
    pub entities: GenesisEntityLineage,
    pub checkpoint: GenesisCheckpoint,
}

pub fn compute_genesis_root() -> GenesisState {
    let timeline = initialize_world_timeline();
    let continuity = initialize_federation_genesis();
    let entities = initialize_entity_lineage();
    let world_root = GenesisWorldRoot {
        world_root: sha256(b"everarcade/genesis/world_root/v1"),
    };
    let checkpoint_root = sha256(b"everarcade/genesis/checkpoint_root/v1");
    let timeline_root = timeline.timeline_0;
    let entity_root = entities.entity_root;
    let continuity_root = continuity.federation_continuity_root;
    let mut state = GenesisState {
        protocol_version: 1,
        world_root,
        checkpoint_root,
        timeline_root,
        entity_root,
        continuity_root,
        timeline,
        continuity,
        entities,
        checkpoint: GenesisCheckpoint {
            world_root: [0; 32],
            timeline_root: [0; 32],
            continuity_root: [0; 32],
            protocol_version: 1,
            topology_epoch: 0,
            checkpoint_root: [0; 32],
        },
    };
    state.checkpoint = create_genesis_checkpoint(&state);
    state
}

pub fn verify_genesis_root(state: &GenesisState) -> Result<(), GenesisError> {
    let expected = compute_genesis_root();
    if &expected != state {
        return Err(GenesisError::Invalid("genesis root mismatch".into()));
    }
    verify_genesis_timeline(&state.timeline)?;
    verify_entity_genesis(&state.entities)?;
    verify_federation_genesis(&state.continuity)?;
    verify_genesis_checkpoint(&state.checkpoint)?;
    Ok(())
}

pub fn replay_genesis() -> GenesisState {
    compute_genesis_root()
}

pub fn verify_genesis_replay(state: &GenesisState) -> Result<(), GenesisError> {
    verify_genesis_root(state)
}
