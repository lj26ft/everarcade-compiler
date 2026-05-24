use crate::world::EvolutionStage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivilizationEntity {
    pub entity_id: String,
    pub owner_id: String,
    pub stage: EvolutionStage,
    pub generation: u64,
}
