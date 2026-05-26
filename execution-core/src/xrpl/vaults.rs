use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VaultDescriptor {
    pub vault_id: String,
    pub owner: String,
    pub lock_until_epoch: u64,
}
