use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SovereignEntity {
    pub entity_id: String,
    pub generation: u64,
    pub identity_root: String,
    pub lineage_root: String,
    pub state_root: String,
}
impl SovereignEntity {
    pub fn genesis(id: impl Into<String>) -> Self {
        let entity_id = id.into();
        let identity_root = format!("entity:{entity_id}:identity");
        let lineage_root = format!("entity:{entity_id}:lineage:0:{identity_root}");
        let state_root = format!("entity:{entity_id}:state:0");
        Self {
            entity_id,
            generation: 0,
            identity_root,
            lineage_root,
            state_root,
        }
    }
}
