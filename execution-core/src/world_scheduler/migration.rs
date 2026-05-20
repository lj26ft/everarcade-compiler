use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityStateBlob {
    pub entity_id: String,
    pub continuity: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityMigrationRecord {
    pub entity_id: String,
    pub from_node: String,
    pub to_node: String,
    pub epoch: u64,
}

pub fn export_entity_state(entity_id: &str, state: &[u8]) -> EntityStateBlob {
    EntityStateBlob {
        entity_id: entity_id.to_string(),
        continuity: state.to_vec(),
    }
}

pub fn import_entity_state(blob: EntityStateBlob) -> EntityStateBlob {
    blob
}

pub fn resume_entity_execution(blob: &EntityStateBlob) -> bool {
    !blob.continuity.is_empty()
}
