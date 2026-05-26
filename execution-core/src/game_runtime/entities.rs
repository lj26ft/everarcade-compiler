use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    pub id: u64,
    pub x: i64,
    pub y: i64,
    pub authority: String,
}
