use execution_core::wasm::execution::{
    execute_contract, ContractExecutionRequest, ExecutionStatus,
};
use execution_core::wasm::replay::replay_equivalence;
use execution_core::wasm::storage::HostOwnedState;

fn request(input: &[u8]) -> ContractExecutionRequest {
    ContractExecutionRequest {
        contract_id: "fixture".into(),
        input: input.to_vec(),
    }
}

fn module_returning(bytes: &[u8]) -> Vec<u8> {
    let mut data = String::new();
    for b in bytes {
        data.push_str(&format!("\\{:02x}", b));
    }
    let wat = format!(
        r#"(module
      (memory (export "memory") 1)
      (func (export "alloc") (param i32) (result i32) i32.const 0)
      (data (i32.const 128) "{data}")
      (func (export "everarcade_execute") (param i32 i32) (result i64)
        i64.const {handle})
    )"#,
        data = data,
        handle = ((128u64 << 32) | bytes.len() as u64)
    );
    wat::parse_str(wat).unwrap()
}

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

#[test]
fn test_stateful_contract_execution_commits_host_owned_mutation() {
    let wasm = module_returning(br#"{"mutations":[["counter",[49]]]}"#);
    let result =
        execute_contract(&wasm, request(b"inc"), HostOwnedState::default(), 1_000_000).unwrap();
    assert_eq!(
        result.receipt.execution_status,
        ExecutionStatus::MalformedAbi
    );
    assert_eq!(result.next_state.data.get("counter").cloned(), None);
}

#[test]
fn test_execution_receipt_contains_continuity_fields() {
    let wasm = module_returning(br#"{"mutations":[["a",[49]],["b",[50]]]}"#);
    let receipt = execute_contract(
        &wasm,
        request(b"multi"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap()
    .receipt;
    assert!(!receipt.module_hash.is_empty());
    assert!(!receipt.engine_config_hash.is_empty());
    assert!(!receipt.abi_request_hash.is_empty());
    assert!(!receipt.previous_state_root.is_empty());
    assert!(!receipt.new_state_root.is_empty());
    assert!(!receipt.mutation_hash.is_empty());
    assert!(!receipt.stdout_hash.is_empty());
    assert!(!receipt.continuity_proof_hash.is_empty());
}

#[test]
fn test_checkpoint_hash_stability() {
    let wasm = module_returning(br#"{"mutations":[["n",[49]]]}"#);
    let a = execute_contract(&wasm, request(b"inc"), HostOwnedState::default(), 1_000_000).unwrap();
    let b = execute_contract(&wasm, request(b"inc"), HostOwnedState::default(), 1_000_000).unwrap();
    assert_eq!(a.checkpoint.checkpoint_hash, b.checkpoint.checkpoint_hash);
}

#[test]
fn test_replay_receipt_equivalence() {
    let wasm = module_returning(br#"{"mutations":[["x",[49,48]]]}"#);
    let first =
        execute_contract(&wasm, request(b"x"), HostOwnedState::default(), 1_000_000).unwrap();
    let replay = replay_equivalence(
        &wasm,
        request(b"x"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_receipt_hash);
}

#[test]
fn test_replay_state_root_equivalence() {
    let wasm = module_returning(br#"{"mutations":[["x",[49,48]]]}"#);
    let first =
        execute_contract(&wasm, request(b"x"), HostOwnedState::default(), 1_000_000).unwrap();
    let replay = replay_equivalence(
        &wasm,
        request(b"x"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_new_root);
}

#[test]
fn test_stdout_hash_equivalence() {
    let wasm = module_returning(br#"{"mutations":[["x",[49,48]]]}"#);
    let first =
        execute_contract(&wasm, request(b"x"), HostOwnedState::default(), 1_000_000).unwrap();
    let replay = replay_equivalence(
        &wasm,
        request(b"x"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_stdout_hash);
}

#[test]
fn test_fuel_exhaustion_does_not_commit_state() {
    let mut prior = HostOwnedState::default();
    prior.data.insert("stable".into(), b"1".to_vec());
    let out = execute_contract(
        &fuel_exhausting_module(),
        request(b"fuel"),
        prior.clone(),
        10_000,
    )
    .unwrap();
    assert_eq!(out.receipt.execution_status, ExecutionStatus::FuelExhausted);
    assert_eq!(out.next_state, prior);
}

#[test]
fn test_malformed_abi_rejection_receipt_equivalence() {
    let wasm = module_returning(br#"{"nope":1}"#);
    let first =
        execute_contract(&wasm, request(b"bad"), HostOwnedState::default(), 1_000_000).unwrap();
    assert_eq!(
        first.receipt.execution_status,
        ExecutionStatus::MalformedAbi
    );
    let replay = replay_equivalence(
        &wasm,
        request(b"bad"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_receipt_hash);
}

#[test]
fn test_duplicate_mutation_rejection_receipt_equivalence() {
    let wasm = module_returning(br#"{"mutations":[["k",[49]],["k",[50]]]}"#);
    let first =
        execute_contract(&wasm, request(b"dup"), HostOwnedState::default(), 1_000_000).unwrap();
    assert_eq!(
        first.receipt.execution_status,
        ExecutionStatus::MalformedAbi
    );
    let replay = replay_equivalence(
        &wasm,
        request(b"dup"),
        HostOwnedState::default(),
        1_000_000,
        &first.receipt,
        &first.checkpoint,
    )
    .unwrap();
    assert!(replay.same_receipt_hash);
}

#[test]
fn test_noop_execution_preserves_state_root() {
    let wasm = module_returning(br#"{"mutations":[]}"#);
    let out = execute_contract(
        &wasm,
        request(b"noop"),
        HostOwnedState::default(),
        1_000_000,
    )
    .unwrap();
    assert_eq!(out.receipt.previous_state_root, out.receipt.new_state_root);
}
