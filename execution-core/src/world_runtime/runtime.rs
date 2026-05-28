use super::{
    recovery::{restore_world, WorldRecoveryRequest},
    tick::execute_world_tick,
    validation::{reject_authority_mutation, validate_world_continuity},
    world::PersistentWorld,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldRuntimeError {
    Divergence,
    UnauthorizedMutation,
}

#[derive(Clone, Debug)]
pub struct PersistentWorldRuntime {
    pub world: PersistentWorld,
    pub replay_history: Vec<String>,
}

impl PersistentWorldRuntime {
    pub fn new(world_id: impl Into<String>) -> Self {
        let world = PersistentWorld::genesis(world_id);
        Self {
            replay_history: vec![world.replay_tip.clone()],
            world,
        }
    }
    pub fn tick(&mut self, input_root: &str) -> Result<(), WorldRuntimeError> {
        let next = execute_world_tick(&self.world, input_root);
        if !validate_world_continuity(&next) {
            return Err(WorldRuntimeError::Divergence);
        }
        self.replay_history.push(next.replay_tip.clone());
        self.world = next;
        Ok(())
    }
    pub fn restore(
        &mut self,
        checkpoint: PersistentWorld,
        replay_restored: PersistentWorld,
    ) -> Result<(), WorldRuntimeError> {
        self.world = restore_world(WorldRecoveryRequest {
            checkpoint,
            replay_restored,
        })
        .map_err(|_| WorldRuntimeError::Divergence)?;
        Ok(())
    }
    pub fn unauthorized_mutation(&self) -> Result<(), WorldRuntimeError> {
        reject_authority_mutation(true).map_err(|_| WorldRuntimeError::UnauthorizedMutation)
    }
}
