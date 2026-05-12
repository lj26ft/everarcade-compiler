use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability {
    pub capability_id: Hash,
    pub issuing_domain: Hash,
    pub authority_scope: Hash,
    pub parent_capability: Option<Hash>,
    pub revocation_root: Option<Hash>,
}
