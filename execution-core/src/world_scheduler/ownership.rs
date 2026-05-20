use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityOwnership {
    pub owner_node: String,
    pub ownership_epoch: u64,
    pub lineage_hash: [u8; 32],
}

#[derive(Debug, Clone, Default)]
pub struct EntityOwnershipRegistry {
    pub entities: BTreeMap<String, EntityOwnership>,
}

pub fn assign_entity_owner(
    registry: &mut EntityOwnershipRegistry,
    entity_id: String,
    owner_node: String,
    lineage_hash: [u8; 32],
) {
    registry.entities.insert(
        entity_id,
        EntityOwnership {
            owner_node,
            ownership_epoch: 0,
            lineage_hash,
        },
    );
}

pub fn transfer_entity_ownership(
    registry: &mut EntityOwnershipRegistry,
    entity_id: &str,
    new_owner: String,
    new_lineage_hash: [u8; 32],
) -> Option<EntityOwnership> {
    let mut item = registry.entities.get(entity_id)?.clone();
    item.owner_node = new_owner;
    item.ownership_epoch = item.ownership_epoch.saturating_add(1);
    item.lineage_hash = new_lineage_hash;
    registry
        .entities
        .insert(entity_id.to_string(), item.clone());
    Some(item)
}

pub fn verify_entity_continuity(previous: &EntityOwnership, current: &EntityOwnership) -> bool {
    current.ownership_epoch >= previous.ownership_epoch && current.lineage_hash != [0u8; 32]
}
