use crate::receipt_runtime::execution_receipt::ExecutionReceipt;

use super::{leaf_hash::leaf_hash, merkle_tree::build_merkle_root, Hash};

fn encode_receipt(receipt: &ExecutionReceipt) -> Vec<u8> {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        receipt.receipt_id,
        receipt.parent_receipt.clone().unwrap_or_default(),
        receipt.execution_root,
        receipt.state_root,
        receipt.graph_root,
        receipt.replay_root,
        receipt.timestamp_index
    )
    .into_bytes()
}

pub fn receipt_leaves(receipts: &[ExecutionReceipt]) -> Vec<Hash> {
    receipts
        .iter()
        .map(|r| leaf_hash(&encode_receipt(r)))
        .collect()
}

pub fn receipt_root(receipts: &[ExecutionReceipt]) -> Hash {
    build_merkle_root(&receipt_leaves(receipts))
}
