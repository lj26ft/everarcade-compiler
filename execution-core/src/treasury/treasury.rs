use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SovereignTreasury {
    pub treasury_id: Hash,
    pub sovereign_domain: Hash,
    pub treasury_root: Hash,
    pub monetary_root: Hash,
    pub fiscal_root: Hash,
}
