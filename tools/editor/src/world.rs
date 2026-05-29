use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldInspection { pub world_id: String, pub entity_count: usize, pub world_root: String }

pub fn inspect_world(world_id: &str, entities: &[&str]) -> WorldInspection {
    let mut sorted = entities.to_vec();
    sorted.sort_unstable();
    WorldInspection { world_id: world_id.to_owned(), entity_count: sorted.len(), world_root: stable_hash(&sorted) }
}
