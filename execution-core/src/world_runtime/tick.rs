use super::world::PersistentWorld;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldTick {
    pub tick: u64,
    pub input_root: String,
    pub output_root: String,
}

pub fn execute_world_tick(world: &PersistentWorld, input_root: &str) -> PersistentWorld {
    let tick = world.tick + 1;
    let state_root = format!(
        "world:{}:state:{tick}:{}:{}",
        world.world_id, world.state_root, input_root
    );
    let replay_tip = format!("world:{}:replay:{tick}:{}", world.world_id, input_root);
    let continuity_root = format!(
        "world:{}:continuity:{tick}:{state_root}:{replay_tip}",
        world.world_id
    );
    PersistentWorld {
        world_id: world.world_id.clone(),
        tick,
        state_root,
        replay_tip,
        continuity_root,
    }
}
