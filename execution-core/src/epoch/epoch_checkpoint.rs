use serde::{Deserialize, Serialize};

use super::epoch::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochCheckpoint {
    pub epoch_index: u64,
    pub checkpoint_root: Hash,
    pub parent_checkpoint: Option<Hash>,
}
