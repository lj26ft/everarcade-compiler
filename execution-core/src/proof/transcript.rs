use crate::hashing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transcript {
    pub protocol_epoch: u64,
    pub labels: Vec<String>,
    pub entries: Vec<String>,
}

pub fn transcript_hash(transcript: &Transcript) -> String {
    hashing::hash_bytes(&bincode::serialize(transcript).expect("transcript serialize failed"))
}
