pub fn entity_lineage(entity_id: &str, parent: Option<&str>) -> String { crate::stable_hash(&["entity-lineage", entity_id, parent.unwrap_or("root")]) }
