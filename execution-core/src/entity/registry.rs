use std::collections::BTreeMap;

use super::identity::EntityIdentity;

#[derive(Default)]
pub struct EntityRegistry {
    by_id: BTreeMap<String, EntityIdentity>,
}

impl EntityRegistry {
    pub fn insert(&mut self, id: EntityIdentity) {
        self.by_id.insert(id.entity_id.clone(), id);
    }
    pub fn get(&self, entity_id: &str) -> Option<&EntityIdentity> {
        self.by_id.get(entity_id)
    }
}
