use execution_core::{hashing, ExecutionNode, ExecutionPlan, VmInput};
use std::collections::BTreeMap;

#[test]
fn test_increment_contract_execution() {
    let input = VmInput { state: BTreeMap::new(), plan: ExecutionPlan { nodes: vec![ExecutionNode { id:"n1".into(), contract_id:"increment".into(), payload: vec![1,2,3], deps: vec![] }] } };
    let a = execution_core::execute::execute_vm(input.clone());
    let b = execution_core::execute::execute_vm(input);
    assert_eq!(a.receipt.node_hashes, b.receipt.node_hashes);
}

#[test]
fn test_set_contract_execution() {
    let input = VmInput { state: BTreeMap::new(), plan: ExecutionPlan { nodes: vec![ExecutionNode { id:"n1".into(), contract_id:"set".into(), payload: vec![9], deps: vec![] }] } };
    let a = execution_core::execute::execute_vm(input.clone());
    let b = execution_core::execute::execute_vm(input);
    assert_eq!(a.receipt.receipt_hash, b.receipt.receipt_hash);
}

#[test]
fn test_contract_hash_stability() {
    let wasm = b"\0asm\x01\0\0\0";
    assert_eq!(hashing::compute_contract_hash(wasm), hashing::compute_contract_hash(wasm));
}

#[test]
fn test_contract_isolation() {
    let mut s = BTreeMap::new();
    s.insert("k".to_string(), "v".to_string());
    let out = execution_core::execute::execute_vm(VmInput { state: s.clone(), plan: ExecutionPlan { nodes: vec![] } });
    assert_eq!(out.updated_state, s);
}

#[test]
fn test_contract_replay() {
    let input = VmInput { state: BTreeMap::new(), plan: ExecutionPlan { nodes: vec![ExecutionNode { id:"n1".into(), contract_id:"set".into(), payload: vec![], deps: vec![] }] } };
    let a = execution_core::execute::execute_vm(input.clone());
    let b = execution_core::execute::execute_vm(input);
    assert_eq!(a.receipt.receipt_hash, b.receipt.receipt_hash);
}
