use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferCheckpoint {
    pub checkpoint_root: Hash,
    pub state_bytes: Vec<u8>,
}

pub fn export_checkpoint_bytes(checkpoint: &TransferCheckpoint) -> Vec<u8> {
    serde_json::to_vec(checkpoint).unwrap_or_default()
}
