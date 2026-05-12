use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionDomain {
    pub domain_id: Hash,
    pub parent_domain: Option<Hash>,
    pub constitutional_root: Hash,
    pub governance_root: Hash,
    pub replay_root: Hash,
}
