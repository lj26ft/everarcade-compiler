use crate::world::{CivilizationEntity, EvolutionStage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityLifecycle {
    pub entity: CivilizationEntity,
    pub history: Vec<EvolutionStage>,
}

impl EntityLifecycle {
    pub fn advance(&mut self, next: EvolutionStage) {
        self.entity.stage = next.clone();
        self.entity.generation = self.entity.generation.saturating_add(1);
        self.history.push(next);
    }
}
