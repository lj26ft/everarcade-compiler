use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerInput {
    pub player_id: String,
    pub dx: i64,
    pub dy: i64,
}
