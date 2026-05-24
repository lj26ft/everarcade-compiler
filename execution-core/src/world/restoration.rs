use crate::world::WorldCheckpoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorationManifest {
    pub world_id: String,
    pub checkpoint: WorldCheckpoint,
    pub cold_restore: bool,
}
