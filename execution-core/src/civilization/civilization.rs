use serde::{Deserialize, Serialize};

use super::genesis::{CivilizationGenesis, Hash};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Civilization {
    pub genesis: CivilizationGenesis,
    pub civilization_root: Hash,
}
