use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocialMemoryRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl SocialMemoryRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("autonomous_world_recovery:social_memory:{id}:lineage:{tick}");
        let continuity_root =
            format!("autonomous_world_recovery:social_memory:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
