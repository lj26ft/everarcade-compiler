use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySummary {
    pub epoch_index: u64,
    pub compressed_replay_root: Hash,
    pub aggregated_receipt_root: Hash,
    pub state_commitment_root: Hash,
}
