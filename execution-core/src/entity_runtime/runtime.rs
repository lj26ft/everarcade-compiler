use super::{
    entity::SovereignEntity,
    evolution::evolve_entity,
    recovery::restore_entity,
    validation::{reject_entity_mutation, validate_entity},
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityRuntimeError {
    Divergence,
    UnauthorizedMutation,
}
#[derive(Clone, Debug)]
pub struct EntityRuntime {
    pub entities: Vec<SovereignEntity>,
}
impl EntityRuntime {
    pub fn new(ids: &[&str]) -> Self {
        Self {
            entities: ids.iter().map(|id| SovereignEntity::genesis(*id)).collect(),
        }
    }
    pub fn evolve_all(&mut self, input: &str) -> Result<(), EntityRuntimeError> {
        let next: Vec<_> = self
            .entities
            .iter()
            .map(|e| evolve_entity(e, input))
            .collect();
        if next.iter().all(validate_entity) {
            self.entities = next;
            Ok(())
        } else {
            Err(EntityRuntimeError::Divergence)
        }
    }
    pub fn restore(
        &self,
        a: &SovereignEntity,
        b: &SovereignEntity,
    ) -> Result<SovereignEntity, EntityRuntimeError> {
        restore_entity(a, b).map_err(|_| EntityRuntimeError::Divergence)
    }
    pub fn unauthorized_mutation(&self) -> Result<(), EntityRuntimeError> {
        reject_entity_mutation(false).map_err(|_| EntityRuntimeError::UnauthorizedMutation)
    }
}
