use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalInterpretation {
    pub interpretation_id: Hash,
    pub constitutional_root: Hash,
    pub interpretation_scope_root: Hash,
    pub lineage_root: Hash,
}
