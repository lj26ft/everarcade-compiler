use serde::{Deserialize, Serialize};

use super::{
    component::ComponentValue, entity::Entity, scheduler, storage::EcsStorage,
    system::DeterministicSystem, validation,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EcsError {
    NonDeterministicSchedule,
    UnauthorizedMutation,
    ReplayDivergence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EcsMutation {
    pub entity_id: String,
    pub component: String,
    pub delta: i64,
    pub authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EcsReplayEvent {
    pub tick: u64,
    pub system_id: String,
    pub mutation: EcsMutation,
    pub resulting_value: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EcsReplayWindow {
    pub events: Vec<EcsReplayEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EcsRuntime {
    pub storage: EcsStorage,
    pub tick: u64,
    pub replay: EcsReplayWindow,
}

impl EcsRuntime {
    pub fn spawn(&mut self, entity: Entity) {
        self.storage.insert_entity(entity);
    }
    pub fn set_component(&mut self, entity_id: &str, component: ComponentValue) {
        self.storage.set_component(entity_id, component);
    }

    pub fn execute_systems(
        &mut self,
        systems: Vec<DeterministicSystem>,
    ) -> Result<EcsReplayWindow, EcsError> {
        if !validation::systems_are_canonical(&systems) {
            return Err(EcsError::NonDeterministicSchedule);
        }
        let ordered_systems = scheduler::deterministic_system_order(systems);
        let entity_ids = scheduler::deterministic_entity_order(self.storage.ordered_entities());
        for system in ordered_systems {
            if system.authority != "deterministic-ecs-runtime" {
                return Err(EcsError::UnauthorizedMutation);
            }
            for entity_id in &entity_ids {
                let current = self
                    .storage
                    .component(entity_id, &system.component)
                    .map(|c| c.value)
                    .unwrap_or(0);
                let resulting_value = current + system.delta;
                let mutation = EcsMutation {
                    entity_id: entity_id.clone(),
                    component: system.component.clone(),
                    delta: system.delta,
                    authority: system.authority.clone(),
                };
                self.apply_mutation(mutation.clone(), resulting_value)?;
                self.replay.events.push(EcsReplayEvent {
                    tick: self.tick,
                    system_id: system.id.clone(),
                    mutation,
                    resulting_value,
                });
            }
        }
        self.tick += 1;
        Ok(self.replay.clone())
    }

    pub fn apply_mutation(
        &mut self,
        mutation: EcsMutation,
        resulting_value: i64,
    ) -> Result<(), EcsError> {
        if mutation.authority != "deterministic-ecs-runtime" {
            return Err(EcsError::UnauthorizedMutation);
        }
        self.storage.set_component(
            &mutation.entity_id,
            ComponentValue::new(mutation.component, resulting_value, mutation.authority),
        );
        Ok(())
    }
}
