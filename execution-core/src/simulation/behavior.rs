use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityBehavior {
    pub entity_id: String,
    pub opcode: String,
    pub intensity: u64,
}
