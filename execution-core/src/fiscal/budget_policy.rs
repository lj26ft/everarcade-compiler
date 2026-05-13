use serde::{Deserialize, Serialize};
use super::fiscal_policy::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetPolicy {
    pub budget_root: Hash,
    pub taxation_root: Hash,
    pub reward_root: Hash,
}
