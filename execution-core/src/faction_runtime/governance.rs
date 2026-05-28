use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl GovernanceRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("faction_runtime:governance:{id}:lineage:{tick}");
        let continuity_root =
            format!("faction_runtime:governance:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
