use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialStructureRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl SocialStructureRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("society_runtime:social_structure:{id}:lineage:{tick}");
        let continuity_root =
            format!("society_runtime:social_structure:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
