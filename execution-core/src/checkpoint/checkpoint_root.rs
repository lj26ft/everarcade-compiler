use crate::{hashing::hash_bytes, merkle::Hash};

use super::checkpoint_snapshot::CheckpointSnapshot;

pub fn compute_checkpoint_root(snapshot: &CheckpointSnapshot) -> Hash {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&snapshot.state_root);
    bytes.extend_from_slice(&snapshot.receipt_root);
    bytes.extend_from_slice(&snapshot.replay_root);
    bytes.extend_from_slice(&snapshot.last_receipt_hash);
    bytes.extend_from_slice(&snapshot.logical_index.to_be_bytes());
    bytes.extend_from_slice(&snapshot.encoded_state);
    let s = hash_bytes(&bytes);
    let mut out = [0u8; 32];
    hex::decode_to_slice(s, &mut out as &mut [u8]).expect("valid hex");
    out
}
