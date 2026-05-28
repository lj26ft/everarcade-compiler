use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeterministicSystem {
    pub id: String,
    pub component: String,
    pub delta: i64,
    pub authority: String,
}

impl DeterministicSystem {
    pub fn new(id: impl Into<String>, component: impl Into<String>, delta: i64) -> Self {
        Self {
            id: id.into(),
            component: component.into(),
            delta,
            authority: "deterministic-ecs-runtime".to_string(),
        }
    }
}
