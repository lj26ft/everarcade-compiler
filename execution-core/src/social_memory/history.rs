use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl HistoryRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("social_memory:history:{id}:lineage:{tick}");
        let continuity_root = format!("social_memory:history:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
