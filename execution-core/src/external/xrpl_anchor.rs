use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct XrplAnchor {
    pub ledger_index: u64,
    pub transaction_hash: Hash,
    pub anchored_root: Hash,
}
