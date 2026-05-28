use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl ResourceRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("ecology_runtime:resource:{id}:lineage:{tick}");
        let continuity_root = format!("ecology_runtime:resource:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
