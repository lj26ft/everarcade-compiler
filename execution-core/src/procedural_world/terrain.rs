use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrainRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl TerrainRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("procedural_world:terrain:{id}:lineage:{tick}");
        let continuity_root = format!("procedural_world:terrain:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
