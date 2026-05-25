use execution_core::wasm::execution::{
    execute_contract, ContractExecutionRequest, ExecutionStatus,
};
use execution_core::wasm::replay::replay_equivalence;
use execution_core::wasm::storage::HostOwnedState;

fn request(op: &str) -> ContractExecutionRequest {
    ContractExecutionRequest {
        contract_id: "fixture".into(),
        input: format!("{{\"op\":\"{}\"}}", op).into_bytes(),
    }
}

#[allow(dead_code)]
fn fuel_exhausting_module() -> Vec<u8> {
    wat::parse_str(
        r#"(module
      (memory (export "memory") 1)
      (func (export "alloc") (param i32) (result i32) i32.const 0)
      (func (export "everarcade_execute") (param i32 i32) (result i64)
        (loop br 0)
        i64.const 0)
    )"#,
    )
    .unwrap()
}

fn guest_runtime_module() -> Vec<u8> {
    let wat = include_str!("fixtures/wasm/guest_runtime.wat");
    wat::parse_str(wat).unwrap()
}

#[test]
fn test_increment_operation_commits_state() {
    let out = execute_contract(
        &guest_runtime_module(),
        request("i"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert_eq!(out.receipt.execution_status, ExecutionStatus::Success);
    assert_eq!(out.next_state.data.get("counter"), Some(&b"1".to_vec()));
}

#[test]
fn test_multi_operation_is_canonically_sorted() {
    let out = execute_contract(
        &guest_runtime_module(),
        request("m"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert_eq!(out.receipt.execution_status, ExecutionStatus::Success);
    let keys: Vec<_> = out.next_state.data.keys().cloned().collect();
    assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn test_noop_preserves_root() {
    let out = execute_contract(
        &guest_runtime_module(),
        request("n"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert_eq!(out.receipt.previous_state_root, out.receipt.new_state_root);
}

#[test]
fn test_duplicate_operation_rejected() {
    let out = execute_contract(
        &guest_runtime_module(),
        request("d"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert!(matches!(
        out.receipt.execution_status,
        ExecutionStatus::DuplicateMutation
    ));
}

#[test]
fn test_malformed_operation_rejected() {
    let out = execute_contract(
        &guest_runtime_module(),
        request("z"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert_eq!(out.receipt.execution_status, ExecutionStatus::MalformedAbi);
}

#[test]
fn test_fuel_operation_rolls_back_state() {
    let mut prior = HostOwnedState::default();
    prior.data.insert("stable".into(), b"1".to_vec());
    let out =
        execute_contract(&guest_runtime_module(), request("f"), prior.clone(), 10_000).unwrap();
    assert_eq!(out.receipt.execution_status, ExecutionStatus::FuelExhausted);
    assert_eq!(out.next_state, prior);
}

#[test]
fn test_stdout_operation_is_replay_equivalent() {
    let first = execute_contract(
        &guest_runtime_module(),
        request("s"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    let replay = replay_equivalence(
        &guest_runtime_module(),
        request("s"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_stdout_hash && replay.same_receipt_hash && replay.same_checkpoint_hash);
}

#[test]
fn test_large_payload_memory_validation() {
    let out = execute_contract(
        &guest_runtime_module(),
        request("l"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert_eq!(out.receipt.execution_status, ExecutionStatus::MalformedAbi);
}

#[test]
fn test_receipt_bytes_are_replay_equivalent() {
    let first = execute_contract(
        &guest_runtime_module(),
        request("i"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    let replay = replay_equivalence(
        &guest_runtime_module(),
        request("i"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_receipt_bytes);
}

#[test]
fn test_checkpoint_lineage_hash_stability() {
    let first = execute_contract(
        &guest_runtime_module(),
        request("i"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    let replay = replay_equivalence(
        &guest_runtime_module(),
        request("i"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_checkpoint_hash);
}

#[test]
fn test_continuity_proof_stability() {
    let first = execute_contract(
        &guest_runtime_module(),
        request("m"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    let replay = replay_equivalence(
        &guest_runtime_module(),
        request("m"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_continuity_proof_hash);
}
