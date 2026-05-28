use super::{validation::validate_world_equivalence, world::PersistentWorld};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRecoveryRequest {
    pub checkpoint: PersistentWorld,
    pub replay_restored: PersistentWorld,
}

pub fn restore_world(req: WorldRecoveryRequest) -> Result<PersistentWorld, &'static str> {
    validate_world_equivalence(&req.checkpoint, &req.replay_restored)?;
    Ok(req.replay_restored)
}
