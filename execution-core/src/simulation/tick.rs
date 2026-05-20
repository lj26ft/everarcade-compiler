use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimulationTick {
    pub tick_id: u64,
    pub simulation_hash: [u8; 32],
    pub interaction_hash: [u8; 32],
    pub entity_state_hash: [u8; 32],
    pub world_state_root: [u8; 32],
}
