use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalPrecedent {
    pub precedent_id: Hash,
    pub constitutional_root: Hash,
    pub interpretation_root: Hash,
    pub lineage_root: Hash,
}
