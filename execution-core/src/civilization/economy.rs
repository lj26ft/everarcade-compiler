use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivilizationEconomy {
    pub ledger_root: String,
    pub supply: u64,
}
impl CivilizationEconomy {
    pub fn genesis(id: &str) -> Self {
        Self {
            ledger_root: format!("civilization:{id}:ledger:0"),
            supply: 100,
        }
    }
}
