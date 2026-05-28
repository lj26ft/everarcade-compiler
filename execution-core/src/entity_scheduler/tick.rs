use super::priority::EntityPriority;
pub fn deterministic_order(mut items: Vec<EntityPriority>) -> Vec<EntityPriority> {
    items.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then(a.entity_id.cmp(&b.entity_id))
    });
    items
}
