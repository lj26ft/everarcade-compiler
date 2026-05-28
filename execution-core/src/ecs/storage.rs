use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{component::ComponentValue, entity::Entity};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EcsStorage {
    pub entities: BTreeMap<String, Entity>,
    pub components: BTreeMap<(String, String), ComponentValue>,
}

impl EcsStorage {
    pub fn insert_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }
    pub fn set_component(&mut self, entity_id: &str, component: ComponentValue) {
        self.components
            .insert((entity_id.to_string(), component.name.clone()), component);
    }
    pub fn component(&self, entity_id: &str, name: &str) -> Option<&ComponentValue> {
        self.components
            .get(&(entity_id.to_string(), name.to_string()))
    }
    pub fn ordered_entities(&self) -> Vec<String> {
        self.entities.keys().cloned().collect()
    }
}
