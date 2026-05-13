use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonetaryPolicy {
    pub monetary_root: Hash,
    pub issuance_root: Hash,
    pub supply_root: Hash,
}
