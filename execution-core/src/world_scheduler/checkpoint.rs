use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldCheckpoint {
    pub tick_id: u64,
    pub checkpoint_root: [u8; 32],
}
