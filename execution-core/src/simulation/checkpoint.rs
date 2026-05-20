use serde::{Deserialize, Serialize};

use crate::simulation::{state::SimulationState, tick::SimulationTick};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimulationCheckpoint {
    pub tick: SimulationTick,
    pub state: SimulationState,
}
