use crate::hashing::hash_bytes;

use super::checkpoint::SettlementCheckpoint;

pub fn checkpoint_hash(checkpoint: &SettlementCheckpoint) -> String {
    let bytes = bincode::serialize(checkpoint).expect("checkpoint serialization failed");
    hash_bytes(&bytes)
}

pub fn xrpl_anchor_payload(checkpoint: &SettlementCheckpoint) -> String {
    format!("everarcade:settlement:{}", checkpoint_hash(checkpoint))
}
