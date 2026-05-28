use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeRecord {
    pub id: String,
    pub lineage: String,
    pub continuity_root: String,
}

impl TradeRecord {
    pub fn deterministic(id: &str, tick: u64) -> Self {
        let lineage = format!("civilization_interaction:trade:{id}:lineage:{tick}");
        let continuity_root =
            format!("civilization_interaction:trade:{id}:continuity:{tick}:{lineage}");
        Self {
            id: id.into(),
            lineage,
            continuity_root,
        }
    }
}
