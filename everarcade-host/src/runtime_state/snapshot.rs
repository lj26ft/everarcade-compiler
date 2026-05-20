use serde::{Deserialize, Serialize};

use super::serialization::RuntimeStateMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStateSnapshot {
    pub checkpoint_sequence: u64,
    pub state_root: [u8; 32],
    pub state: RuntimeStateMap,
}
