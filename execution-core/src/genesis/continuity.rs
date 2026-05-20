use serde::{Deserialize, Serialize};

use crate::genesis::error::GenesisError;
use crate::hashing::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisContinuity {
    pub topology_epoch: u64,
    pub federation_continuity_root: [u8; 32],
    pub lease_continuity_root: [u8; 32],
    pub synchronization_lineage_root: [u8; 32],
}

pub fn initialize_federation_genesis() -> GenesisContinuity {
    GenesisContinuity {
        topology_epoch: 0,
        federation_continuity_root: sha256(b"everarcade/genesis/federation_continuity/v1"),
        lease_continuity_root: sha256(b"everarcade/genesis/lease_continuity/v1"),
        synchronization_lineage_root: sha256(b"everarcade/genesis/synchronization_lineage/v1"),
    }
}

pub fn verify_federation_genesis(continuity: &GenesisContinuity) -> Result<(), GenesisError> {
    if continuity == &initialize_federation_genesis() {
        Ok(())
    } else {
        Err(GenesisError::Invalid("federation genesis mismatch".into()))
    }
}

pub fn inspect_bootstrap_continuity(
    state: &crate::genesis::genesis::GenesisState,
) -> GenesisContinuity {
    state.continuity.clone()
}
