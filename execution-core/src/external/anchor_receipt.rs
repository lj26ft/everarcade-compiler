use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalAnchorReceipt {
    pub anchor_root: Hash,
    pub xrpl_anchor_root: Option<Hash>,
    pub evernode_anchor_root: Option<Hash>,
}
