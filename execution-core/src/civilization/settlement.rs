use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settlement {
    pub settlement_id: String,
    pub population: u64,
    pub resource_root: String,
}
impl Settlement {
    pub fn new(id: &str) -> Self {
        Self {
            settlement_id: id.into(),
            population: 1,
            resource_root: format!("settlement:{id}:resources:0"),
        }
    }
}
