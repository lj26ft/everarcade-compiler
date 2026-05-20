use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldEntity {
    pub entity_id: String,
    pub continuity_hash: [u8; 32],
}
