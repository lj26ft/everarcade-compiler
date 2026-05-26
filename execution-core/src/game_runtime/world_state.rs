use std::collections::BTreeMap;
use super::entities::Entity;
#[derive(Debug, Clone, PartialEq)]
pub struct WorldState { pub tick: u64, pub entities: BTreeMap<u64, Entity> }
impl WorldState { pub fn new() -> Self { Self { tick: 0, entities: BTreeMap::new() } } }
