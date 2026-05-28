use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCheckpoint {
    pub checkpoint_root: String,
    pub replay_tip: String,
}
