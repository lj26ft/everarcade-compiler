use super::world::PersistentWorld;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldContinuity {
    pub world_id: String,
    pub tick: u64,
    pub state_root: String,
    pub replay_tip: String,
    pub continuity_root: String,
}

impl From<&PersistentWorld> for WorldContinuity {
    fn from(w: &PersistentWorld) -> Self {
        Self {
            world_id: w.world_id.clone(),
            tick: w.tick,
            state_root: w.state_root.clone(),
            replay_tip: w.replay_tip.clone(),
            continuity_root: w.continuity_root.clone(),
        }
    }
}
