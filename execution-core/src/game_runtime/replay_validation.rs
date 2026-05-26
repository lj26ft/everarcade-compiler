use serde::{Deserialize, Serialize};

use super::replay_runtime::ReplayRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayValidationResult {
    pub ok: bool,
    pub rejection_root: Option<String>,
}

pub fn verify_replay(expected: &ReplayRecord, actual: &ReplayRecord) -> ReplayValidationResult {
    if expected == actual {
        ReplayValidationResult {
            ok: true,
            rejection_root: None,
        }
    } else {
        let root = execution_core_rejection_root(expected, actual);
        ReplayValidationResult {
            ok: false,
            rejection_root: Some(root),
        }
    }
}

fn execution_core_rejection_root(expected: &ReplayRecord, actual: &ReplayRecord) -> String {
    let e = serde_json::to_vec(expected).unwrap_or_default();
    let a = serde_json::to_vec(actual).unwrap_or_default();
    crate::hashing::hash_bytes(&[e, a].concat())
}
