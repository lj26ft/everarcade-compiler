use crate::distributed_receipts::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedReceiptManifest {
    pub receipt_count: u64,
    pub latest_receipt_root: Option<Hash>,
    pub latest_replay_root: Option<Hash>,
    pub latest_checkpoint_root: Option<Hash>,
}
