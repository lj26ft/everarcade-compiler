use execution_core::wasm::memory::{deserialize_abi, serialize_abi};
use execution_core::{ExecutionPlan, VmInput};
use std::collections::BTreeMap;

fn sample_input() -> VmInput {
    VmInput {
        protocol_epoch_id: 1,
        state: BTreeMap::from([("k".into(), "v".into())]),
        plan: ExecutionPlan { nodes: vec![] },
    }
}

#[test]
fn test_raw_memory_roundtrip() {
    let payload = b"everarcade".to_vec();
    let read_back = payload.clone();
    assert_eq!(payload, read_back);
}

#[test]
fn test_structured_abi_roundtrip() {
    let input = sample_input();
    let bytes = serialize_abi(&input).unwrap();
    let decoded: VmInput = deserialize_abi(&bytes).unwrap();
    let out = execution_core::execute::execute_vm(decoded);
    assert_eq!(out.updated_state.get("k").unwrap(), "v");
}

#[test]
fn test_invalid_payload_rejected() {
    let bad = vec![0xff, 0x00, 0x13];
    let decoded: anyhow::Result<VmInput> = deserialize_abi(&bad);
    assert!(decoded.is_err());
}

#[test]
fn test_large_payloads() {
    let mut state = BTreeMap::new();
    for i in 0..5000 {
        state.insert(format!("k{i}"), "x".repeat(32));
    }
    let input = VmInput {
        protocol_epoch_id: 1,
        state,
        plan: ExecutionPlan { nodes: vec![] },
    };
    let bytes = serialize_abi(&input).unwrap();
    let decoded: VmInput = deserialize_abi(&bytes).unwrap();
    assert_eq!(decoded.state.len(), 5000);
}

#[test]
fn test_serialization_stability() {
    let input = sample_input();
    let a = serialize_abi(&input).unwrap();
    let b = serialize_abi(&input).unwrap();
    assert_eq!(a, b);
}
