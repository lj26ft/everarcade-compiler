use crate::distributed_sync::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedReceiptRequest {
    pub package_root: Hash,
    pub from_replay_root: Hash,
    pub target_replay_root: Option<Hash>,
}
