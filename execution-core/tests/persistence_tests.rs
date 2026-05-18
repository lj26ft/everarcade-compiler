use execution_core::persistence::{
    checkpoint_store, package_store, receipt_store, restore_and_replay,
};
use execution_core::vm::{execute_vm_boundary, VmExecutionInput};
use std::fs;

fn sample_receipt() -> execution_core::vm::VmExecutionReceipt {
    let input = VmExecutionInput {
        package_manifest_root: [1; 32],
        civilization_root: [1; 32],
        pre_state_root: [2; 32],
        prior_replay_root_value: [2; 32],
        checkpoint_root: checkpoint_store::checkpoint_root(b"state-before"),
        payload_root: checkpoint_store::checkpoint_root(b"state-before"),
    };
    execute_vm_boundary(&input).0
}

#[test]
fn test_save_load_receipt_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("receipt.bin");
    let receipt = sample_receipt();
    receipt_store::save_receipt(&path, &receipt).unwrap();
    assert_eq!(receipt_store::load_receipt(&path).unwrap(), receipt);
}

#[test]
fn test_save_load_checkpoint_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cp.bin");
    checkpoint_store::save_checkpoint(&path, b"state-before").unwrap();
    let expected = checkpoint_store::checkpoint_root(b"state-before");
    assert_eq!(
        checkpoint_store::load_checkpoint(&path, Some(expected)).unwrap(),
        b"state-before"
    );
}

#[test]
fn test_checkpoint_root_mismatch_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("cp.bin");
    checkpoint_store::save_checkpoint(&path, b"state-before").unwrap();
    assert!(checkpoint_store::load_checkpoint(&path, Some([9; 32])).is_err());
}

#[test]
fn test_save_load_package_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("w.wasm");
    package_store::save_package(&path, b"\0asm").unwrap();
    let root = package_store::package_root(b"\0asm");
    assert_eq!(package_store::load_package(&path, root).unwrap(), b"\0asm");
}

#[test]
fn test_package_root_mismatch_fails() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("w.wasm");
    package_store::save_package(&path, b"\0asm").unwrap();
    assert!(package_store::load_package(&path, [7; 32]).is_err());
}

#[test]
fn test_restore_verify_success() {
    let dir = tempfile::tempdir().unwrap();
    let cp = dir.path().join("cp.bin");
    let rp = dir.path().join("r.bin");
    let pp = dir.path().join("p.wasm");
    let pkg = b"wasm-package";
    package_store::save_package(&pp, pkg).unwrap();
    checkpoint_store::save_checkpoint(&cp, b"state-before").unwrap();
    let mut receipt = sample_receipt();
    receipt.package_root = package_store::package_root(pkg);
    receipt_store::save_receipt(&rp, &receipt).unwrap();
    let out = restore_and_replay(&pp, &rp, &cp).unwrap();
    assert!(out.state_match);
}

#[test]
fn test_restore_verify_tampered_checkpoint_fails() {
    let dir = tempfile::tempdir().unwrap();
    let cp = dir.path().join("cp.bin");
    let rp = dir.path().join("r.bin");
    let pp = dir.path().join("p.wasm");
    let pkg = b"wasm-package";
    package_store::save_package(&pp, pkg).unwrap();
    checkpoint_store::save_checkpoint(&cp, b"state-before").unwrap();
    let mut receipt = sample_receipt();
    receipt.package_root = package_store::package_root(pkg);
    receipt_store::save_receipt(&rp, &receipt).unwrap();
    fs::write(&cp, b"tampered").unwrap();
    assert!(restore_and_replay(&pp, &rp, &cp).is_err());
}

#[test]
fn test_restore_verify_tampered_receipt_fails() {
    let dir = tempfile::tempdir().unwrap();
    let cp = dir.path().join("cp.bin");
    let rp = dir.path().join("r.bin");
    let pp = dir.path().join("p.wasm");
    let pkg = b"wasm-package";
    package_store::save_package(&pp, pkg).unwrap();
    checkpoint_store::save_checkpoint(&cp, b"state-before").unwrap();
    let mut receipt = sample_receipt();
    receipt.package_root = package_store::package_root(pkg);
    receipt.next_replay_root = [0; 32];
    receipt_store::save_receipt(&rp, &receipt).unwrap();
    let out = restore_and_replay(&pp, &rp, &cp).unwrap();
    assert!(!out.receipt_match);
}
