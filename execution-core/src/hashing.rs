// FILE: execution-core/src/hashing.rs

use crate::ExecutionNode;

use sha2::{Digest, Sha256};

use std::collections::BTreeMap;

//
// ============================================================
// SHA256 HELPER
// ============================================================
//

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    hex::encode(result)
}

//
// ============================================================
// STATE ROOT
// ============================================================
//

pub fn compute_state_root(
    state: &BTreeMap<String, String>,
) -> String {
    let canonical =
        serde_json::to_vec(state)
            .expect("state serialize failed");

    sha256(&canonical)
}

//
// ============================================================
// NODE HASH
// ============================================================
//

pub fn compute_node_hash(
    node: &ExecutionNode,
) -> String {
    let canonical =
        serde_json::to_vec(node)
            .expect("node serialize failed");

    sha256(&canonical)
}

//
// ============================================================
// EXECUTION ROOT
// ============================================================
//

pub fn compute_execution_root(
    node_hashes: &BTreeMap<String, String>,
) -> String {
    let canonical =
        serde_json::to_vec(node_hashes)
            .expect("execution serialize failed");

    sha256(&canonical)
}

//
// ============================================================
// RECEIPT HASH
// ============================================================
//

pub fn compute_receipt_hash(
    previous_state_root: &str,
    new_state_root: &str,
    execution_root: &str,
) -> String {
    let combined = format!(
        "{}{}{}",
        previous_state_root,
        new_state_root,
        execution_root
    );

    sha256(combined.as_bytes())
}
