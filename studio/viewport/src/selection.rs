pub fn select_entity(projection_hash: &str, entity_id: &str) -> String { crate::stable_hash(&["selection", projection_hash, entity_id]) }
