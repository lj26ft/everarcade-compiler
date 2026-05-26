use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entity {
    pub id: u64,
    pub x: i64,
    pub y: i64,
    pub authority: String,
    pub runtime_lineage: String,
    pub world_continuity: String,
}

impl Entity {
    pub fn apply_bounded_movement(&mut self, dx: i64, dy: i64, bounds: (i64, i64, i64, i64)) {
        let (min_x, max_x, min_y, max_y) = bounds;
        self.x = (self.x + dx).clamp(min_x, max_x);
        self.y = (self.y + dy).clamp(min_y, max_y);
    }
}
