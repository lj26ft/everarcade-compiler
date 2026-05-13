use serde::{Deserialize, Serialize};
use super::treasury::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreasuryCheckpoint {
    pub checkpoint_id: Hash,
    pub treasury_root: Hash,
    pub lineage_root: Hash,
}
