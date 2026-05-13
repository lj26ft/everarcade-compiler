use serde::{Deserialize, Serialize};

use super::genesis::{CivilizationGenesis, Hash};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivilizationPackage {
    pub genesis: CivilizationGenesis,
    pub execution_root: Hash,
    pub replay_root: Hash,
    pub proof_root: Hash,
    pub checkpoint_root: Hash,
}
