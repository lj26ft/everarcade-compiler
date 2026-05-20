use std::collections::BTreeMap;

pub fn route_entity(
    region_by_entity: &BTreeMap<String, String>,
    entity_id: &str,
) -> Option<String> {
    region_by_entity.get(entity_id).cloned()
}
