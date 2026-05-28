use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl MigrationRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("civilization_interaction:migration:{id}:lineage:{tick}");
        let continuity_root =
            format!("civilization_interaction:migration:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
