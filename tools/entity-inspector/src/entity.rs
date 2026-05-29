use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityInspection { pub entity_id: String, pub lineage: String }

pub fn inspect_entity(entity_id: &str, components: &[&str]) -> EntityInspection { EntityInspection { entity_id: entity_id.to_owned(), lineage: stable_hash(components) } }
