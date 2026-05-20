use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimulationState {
    pub tick_id: u64,
    pub entities: BTreeMap<String, u64>,
    pub inventory: BTreeMap<String, u64>,
    pub economy_volume: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimulationContinuity {
    pub last_tick: u64,
    pub world_state_root: [u8; 32],
}
