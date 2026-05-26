use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldIdentity {
    pub world_id: String,
    pub world_name: String,
    pub governance_key: String,
}
