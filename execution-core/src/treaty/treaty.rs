use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionTreaty {
    pub treaty_id: Hash,
    pub participating_domains: Vec<Hash>,
    pub constitutional_scope_root: Hash,
    pub capability_scope_root: Hash,
    pub arbitration_root: Hash,
}
