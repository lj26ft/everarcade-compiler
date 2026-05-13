use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementAnchor {
    pub anchored_root: Hash,
    pub external_commitment: Hash,
}
