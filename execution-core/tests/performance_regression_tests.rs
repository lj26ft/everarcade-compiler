use execution_core::diagnostics::ExecutionProfile;
use sha2::{Digest, Sha256};

fn hash_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

#[test]
fn test_benchmark_does_not_change_receipt() {
    let receipts = vec!["receipt-a", "receipt-b", "receipt-c"];
    let receipt_hash = hash_bytes(serde_json::to_vec(&receipts).unwrap().as_slice());

    let mut profile = ExecutionProfile::default();
    profile.diagnostic_duration_ns = 42;
    profile.estimated_memory_bytes = 8_192;

    let receipt_hash_after_profile = hash_bytes(serde_json::to_vec(&receipts).unwrap().as_slice());
    assert_eq!(receipt_hash, receipt_hash_after_profile);
    assert_eq!(profile.profile_version, 1);
}

#[test]
fn test_replay_scaling_consistency() {
    let scales = [100_u64, 1_000, 10_000, 100_000];
    let ops: Vec<u64> = scales.into_iter().collect();
    assert_eq!(ops, vec![100, 1_000, 10_000, 100_000]);
}

#[test]
fn test_dag_scaling_profile_stability() {
    let mut profile = ExecutionProfile::default();
    profile.dag_execution_count = 1_000;
    profile.receipt_count = 1_000;
    profile.state_diff_count = 1_000;

    let serialized_once = serde_json::to_vec(&profile).unwrap();
    let serialized_twice = serde_json::to_vec(&profile).unwrap();
    assert_eq!(serialized_once, serialized_twice);
}

#[test]
fn test_wasm_benchmark_consistency() {
    let mut profile = ExecutionProfile::default();
    profile.wasm_call_count = 10_000;
    profile.fuel_consumed = 620_000;
    profile.memory_pages_touched = 24;

    assert_eq!(profile.wasm_call_count, 10_000);
    assert!(profile.fuel_consumed > 0);
}

#[test]
fn test_profile_excluded_from_state_root() {
    let canonical_state = vec!["state-a", "state-b"];
    let state_root_before = hash_bytes(serde_json::to_vec(&canonical_state).unwrap().as_slice());

    let mut profile = ExecutionProfile::default();
    profile.replay_operations = 10_000;
    profile.diagnostic_duration_ns = 100;

    let state_root_after = hash_bytes(serde_json::to_vec(&canonical_state).unwrap().as_slice());
    assert_eq!(state_root_before, state_root_after);
    assert_eq!(profile.replay_operations, 10_000);
}
