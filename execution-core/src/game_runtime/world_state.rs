use super::entities::Entity;
use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq)]
pub struct WorldState {
    pub tick: u64,
    pub entities: BTreeMap<u64, Entity>,
}
impl WorldState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            entities: BTreeMap::new(),
        }
    }
}
