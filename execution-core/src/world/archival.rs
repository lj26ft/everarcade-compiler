use crate::world::WorldCheckpoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivilizationArchive {
    pub era: u64,
    pub checkpoints: Vec<WorldCheckpoint>,
}
