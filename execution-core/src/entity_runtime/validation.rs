use super::{entity::SovereignEntity, identity::deterministic_identity, lineage::lineage_root};
pub fn validate_entity(e: &SovereignEntity) -> bool {
    e.identity_root == deterministic_identity(&e.entity_id)
        && e.lineage_root == lineage_root(&e.entity_id, e.generation, &e.identity_root)
}
pub fn reject_entity_mutation(authorized: bool) -> Result<(), &'static str> {
    if authorized {
        Ok(())
    } else {
        Err("unauthorized entity mutation rejected")
    }
}
pub fn validate_entity_equivalence(
    a: &SovereignEntity,
    b: &SovereignEntity,
) -> Result<(), &'static str> {
    if a == b && validate_entity(a) {
        Ok(())
    } else {
        Err("entity divergence rejected")
    }
}
