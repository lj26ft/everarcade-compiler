use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    pub id: String,
    pub lineage: Vec<String>,
}

impl Entity {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            lineage: Vec::new(),
        }
    }
}
