use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CultureRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl CultureRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("society_runtime:culture:{id}:lineage:{tick}");
        let continuity_root = format!("society_runtime:culture:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
