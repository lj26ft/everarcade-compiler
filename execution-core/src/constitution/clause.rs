use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalClause {
    pub clause_id: Hash,
    pub constitutional_scope_root: Hash,
    pub execution_root: Hash,
    pub lineage_root: Hash,
}
