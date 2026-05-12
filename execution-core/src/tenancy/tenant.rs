use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionTenant {
    pub tenant_id: Hash,
    pub domain_id: Hash,
    pub budget_root: Hash,
    pub replay_root: Hash,
}
