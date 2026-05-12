use crate::execution::ExecutionState;

use super::{leaf_hash::leaf_hash, merkle_tree::build_merkle_root, Hash};

pub fn state_leaves(state: &ExecutionState) -> Vec<Hash> {
    state
        .values
        .iter()
        .map(|(k, v)| {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&(k.len() as u64).to_be_bytes());
            bytes.extend_from_slice(k.as_bytes());
            bytes.extend_from_slice(&(v.len() as u64).to_be_bytes());
            bytes.extend_from_slice(v);
            leaf_hash(&bytes)
        })
        .collect()
}

pub fn state_root(state: &ExecutionState) -> Hash {
    build_merkle_root(&state_leaves(state))
}
