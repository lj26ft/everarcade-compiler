use super::{entity::SovereignEntity, lineage::lineage_root};
pub fn evolve_entity(entity: &SovereignEntity, input_root: &str) -> SovereignEntity {
    let generation = entity.generation + 1;
    let state_root = format!(
        "entity:{}:state:{generation}:{}:{}",
        entity.entity_id, entity.state_root, input_root
    );
    let lineage_root = lineage_root(&entity.entity_id, generation, &entity.identity_root);
    SovereignEntity {
        entity_id: entity.entity_id.clone(),
        generation,
        identity_root: entity.identity_root.clone(),
        lineage_root,
        state_root,
    }
}
