use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalAmendment {
    pub amendment_id: Hash,
    pub prior_constitution_root: Hash,
    pub next_constitution_root: Hash,
    pub amendment_lineage_root: Hash,
}
