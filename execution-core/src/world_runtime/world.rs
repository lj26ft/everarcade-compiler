use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentWorld {
    pub world_id: String,
    pub tick: u64,
    pub state_root: String,
    pub replay_tip: String,
    pub continuity_root: String,
}

impl PersistentWorld {
    pub fn genesis(world_id: impl Into<String>) -> Self {
        let world_id = world_id.into();
        let state_root = format!("world:{world_id}:state:0");
        let replay_tip = format!("world:{world_id}:replay:0");
        let continuity_root = format!("world:{world_id}:continuity:0:{state_root}:{replay_tip}");
        Self {
            world_id,
            tick: 0,
            state_root,
            replay_tip,
            continuity_root,
        }
    }
}
