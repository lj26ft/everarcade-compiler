use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiscalPolicy {
    pub policy_id: Hash,
    pub constitutional_root: Hash,
    pub fiscal_root: Hash,
    pub lineage_root: Hash,
}
