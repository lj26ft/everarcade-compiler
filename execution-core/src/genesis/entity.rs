use serde::{Deserialize, Serialize};

use crate::genesis::error::GenesisError;
use crate::hashing::sha256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisEntityLineage {
    pub entity_root: [u8; 32],
    pub ownership_root: [u8; 32],
    pub migration_root: [u8; 32],
}

pub fn initialize_entity_lineage() -> GenesisEntityLineage {
    GenesisEntityLineage {
        entity_root: sha256(b"everarcade/genesis/entity_root/v1"),
        ownership_root: sha256(b"everarcade/genesis/ownership_root/v1"),
        migration_root: sha256(b"everarcade/genesis/migration_root/v1"),
    }
}

pub fn verify_entity_genesis(lineage: &GenesisEntityLineage) -> Result<(), GenesisError> {
    if lineage == &initialize_entity_lineage() {
        Ok(())
    } else {
        Err(GenesisError::Invalid("entity genesis mismatch".into()))
    }
}
