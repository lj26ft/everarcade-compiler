use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XrplAccount {
    pub address: String,
    pub tag: Option<u32>,
    pub sequence: u64,
}
