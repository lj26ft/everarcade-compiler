use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TerrainCell {
    pub x: i32,
    pub y: i32,
    pub height: i64,
    pub biome: String,
}
