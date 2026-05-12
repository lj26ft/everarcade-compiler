use serde::{Deserialize, Serialize};

use super::epoch::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochSummary {
    pub epoch_index: u64,
    pub epoch_root: Hash,
    pub replay_root: Hash,
    pub receipt_root: Hash,
}
