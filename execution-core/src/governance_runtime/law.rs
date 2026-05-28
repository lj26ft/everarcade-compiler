use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl LawRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("governance_runtime:law:{id}:lineage:{tick}");
        let continuity_root = format!("governance_runtime:law:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
