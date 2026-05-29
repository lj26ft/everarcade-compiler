pub fn inspect_entity(entity_id: &str, projection_hash: &str) -> String { crate::stable_hash(&["inspect-entity", entity_id, projection_hash]) }
