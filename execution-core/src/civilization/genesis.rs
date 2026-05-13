use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivilizationGenesis {
    pub civilization_id: Hash,
    pub domain_root: Hash,
    pub constitution_root: Hash,
    pub treasury_root: Hash,
    pub fiscal_root: Hash,
    pub monetary_root: Hash,
    pub asset_root: Hash,
}
