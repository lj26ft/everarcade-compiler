use crate::{ExecutionNode, ExecutionReceipt};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub fn hash_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub fn compute_state_root(state: &BTreeMap<String, String>) -> String {
    hash_bytes(&bincode::serialize(state).expect("state serialize failed"))
}

pub fn compute_node_hash(node: &ExecutionNode) -> String {
    hash_bytes(&bincode::serialize(node).expect("node serialize failed"))
}

pub fn compute_execution_root(node_hashes: &BTreeMap<String, String>) -> String {
    hash_bytes(&bincode::serialize(node_hashes).expect("execution root serialize failed"))
}

pub fn compute_contract_hash(wasm: &[u8]) -> String {
    hash_bytes(wasm)
}

pub fn compute_receipt_hash(receipt: &ExecutionReceipt) -> String {
    let mut clone = receipt.clone();
    clone.receipt_hash.clear();
    hash_bytes(&bincode::serialize(&clone).expect("receipt serialize failed"))
}
