use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceCapability {
    pub capability_id: Hash,
    pub constitutional_scope: Hash,
}
