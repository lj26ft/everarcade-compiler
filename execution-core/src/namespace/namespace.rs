use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionNamespace {
    pub namespace_id: Hash,
    pub owning_domain: Hash,
    pub delegation_root: Hash,
}
