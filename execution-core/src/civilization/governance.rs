use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceState {
    pub governance_root: String,
    pub epoch: u64,
}
impl GovernanceState {
    pub fn genesis(id: &str) -> Self {
        Self {
            governance_root: format!("civilization:{id}:governance:0"),
            epoch: 0,
        }
    }
}
