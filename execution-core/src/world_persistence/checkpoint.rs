use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldCheckpoint {
    pub tick: u64,
    pub world_root: String,
    pub replay_tip: String,
    pub checkpoint_root: String,
}
impl WorldCheckpoint {
    pub fn new(tick: u64, world_root: &str, replay_tip: &str) -> Self {
        Self {
            tick,
            world_root: world_root.into(),
            replay_tip: replay_tip.into(),
            checkpoint_root: format!("checkpoint:{tick}:{world_root}:{replay_tip}"),
        }
    }
}
