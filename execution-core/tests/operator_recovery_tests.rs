use std::{fs, path::PathBuf};

use execution_core::{
    canonical::encoding::{canonical_decode, canonical_encode},
    lineage::ExecutionLineageChain,
    operator::{
        descriptor_hash, load_recovery_descriptor, recover_world, save_recovery_descriptor,
        OperatorRecoveryError, OperatorRecoveryInput, WorldRecoveryDescriptor,
    },
    persistence::receipt_store,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from("everarcade-host/tests/fixtures/counter_world").join(name)
}

fn setup_fixture(dir: &std::path::Path) -> OperatorRecoveryInput {
    let package_path = dir.join("world.wasm");
    let checkpoint_path = dir.join("checkpoint_0.bin");
    let lineage_path = dir.join("lineage.bin");
    let receipt_1 = dir.join("receipt_1.bin");
    let receipt_2 = dir.join("receipt_2.bin");
    for (src, dst) in [
        (fixture("world.wasm"), &package_path),
        (fixture("checkpoint_0.bin"), &checkpoint_path),
        (fixture("lineage.bin"), &lineage_path),
        (fixture("receipt_1.bin"), &receipt_1),
        (fixture("receipt_2.bin"), &receipt_2),
    ] {
        fs::copy(src, dst).unwrap();
    }
    OperatorRecoveryInput {
        package_path,
        checkpoint_path,
        lineage_path,
        receipt_paths: vec![receipt_1, receipt_2],
        descriptor_output_path: dir.join("recovery_descriptor.bin"),
    }
}

#[test]
fn test_world_recovery_success() {
    let tmp = tempfile::tempdir().unwrap();
    let out = recover_world(setup_fixture(tmp.path())).unwrap();
    assert!(out.report.recovery_ok);
}

#[test]
fn test_world_recovery_checkpoint_mismatch_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let input = setup_fixture(tmp.path());
    let mut bytes = fs::read(&input.checkpoint_path).unwrap();
    bytes[0] ^= 0xff;
    fs::write(&input.checkpoint_path, bytes).unwrap();
    let err = recover_world(input).unwrap_err();
    match err {
        OperatorRecoveryError::Validation(m) => {
            assert!(m.field == "checkpoint_root" || m.field == "pre_state_root");
            assert!(!m.expected.is_empty());
            assert!(!m.actual.is_empty());
        }
        _ => panic!("expected validation error"),
    }
}

#[test]
fn test_world_recovery_manifest_mismatch_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let input = setup_fixture(tmp.path());
    let _ = recover_world(input.clone()).unwrap();
    let manifest_path = input
        .descriptor_output_path
        .parent()
        .unwrap()
        .join("manifest.bin");
    let mut bytes = fs::read(&manifest_path).unwrap();
    bytes[0] ^= 0x01;
    fs::write(&manifest_path, bytes).unwrap();
    let err = recover_world(input).unwrap_err();
    match err {
        OperatorRecoveryError::Validation(m) => {
            assert_eq!(m.field, "manifest_hash");
            assert!(!m.expected.is_empty());
            assert!(!m.actual.is_empty());
        }
        _ => panic!("expected validation error"),
    }
}

#[test]
fn test_world_recovery_lineage_mismatch_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let input = setup_fixture(tmp.path());
    let bytes = fs::read(&input.lineage_path).unwrap();
    let mut chain: ExecutionLineageChain = canonical_decode(&bytes).unwrap();
    chain.records[1].previous_execution_id = Some([0u8; 32]);
    fs::write(&input.lineage_path, canonical_encode(&chain).unwrap()).unwrap();
    let err = recover_world(input).unwrap_err();
    match err {
        OperatorRecoveryError::Validation(m) => {
            assert_eq!(m.field, "previous_execution_id");
            assert!(!m.expected.is_empty());
            assert!(!m.actual.is_empty());
        }
        _ => panic!("expected validation error"),
    }
}

#[test]
fn test_world_recovery_replay_mismatch_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let input = setup_fixture(tmp.path());
    let mut receipt = receipt_store::load_receipt(&input.receipt_paths[1]).unwrap();
    receipt.execution_root[0] ^= 0x80;
    receipt_store::save_receipt(&input.receipt_paths[1], &receipt).unwrap();
    let err = recover_world(input).unwrap_err();
    match err {
        OperatorRecoveryError::Validation(m) => {
            assert!(["execution_id", "receipt_execution_id", "receipt"].contains(&m.field.as_str()));
            assert!(!m.expected.is_empty());
            assert!(!m.actual.is_empty());
        }
        _ => panic!("expected validation error"),
    }
}

#[test]
fn test_recovery_descriptor_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let d = WorldRecoveryDescriptor {
        world_id: [1; 32],
        package_root: [2; 32],
        latest_checkpoint_root: [3; 32],
        latest_execution_id: [4; 32],
        manifest_hash: [5; 32],
    };
    let path = tmp.path().join("d.bin");
    save_recovery_descriptor(&path, &d).unwrap();
    let got = load_recovery_descriptor(&path).unwrap();
    assert_eq!(d, got);
}

#[test]
fn test_recovery_descriptor_envelope_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let d = WorldRecoveryDescriptor {
        world_id: [6; 32],
        package_root: [7; 32],
        latest_checkpoint_root: [8; 32],
        latest_execution_id: [9; 32],
        manifest_hash: [10; 32],
    };
    let path = tmp.path().join("d2.bin");
    save_recovery_descriptor(&path, &d).unwrap();
    assert_eq!(
        descriptor_hash(&d),
        descriptor_hash(&load_recovery_descriptor(&path).unwrap())
    );
}

#[test]
fn test_verify_recovery_descriptor_tamper_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let d = WorldRecoveryDescriptor {
        world_id: [1; 32],
        package_root: [2; 32],
        latest_checkpoint_root: [3; 32],
        latest_execution_id: [4; 32],
        manifest_hash: [5; 32],
    };
    let path = tmp.path().join("tampered.bin");
    save_recovery_descriptor(&path, &d).unwrap();
    let mut bytes = fs::read(&path).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0x01;
    fs::write(&path, bytes).unwrap();
    let err = load_recovery_descriptor(&path).unwrap_err();
    match err {
        OperatorRecoveryError::Validation(m) => assert_eq!(m.field, "descriptor_hash"),
        _ => panic!("expected validation"),
    }
}

#[test]
fn test_descriptor_hash_stable() {
    let d = WorldRecoveryDescriptor {
        world_id: [9; 32],
        package_root: [2; 32],
        latest_checkpoint_root: [3; 32],
        latest_execution_id: [4; 32],
        manifest_hash: [5; 32],
    };
    assert_eq!(descriptor_hash(&d), descriptor_hash(&d));
}

#[test]
fn test_cross_run_recovery_equivalence() {
    let tmp1 = tempfile::tempdir().unwrap();
    let tmp2 = tempfile::tempdir().unwrap();
    let a = recover_world(setup_fixture(tmp1.path())).unwrap();
    let b = recover_world(setup_fixture(tmp2.path())).unwrap();
    assert_eq!(a.descriptor_hash, b.descriptor_hash);
    assert_eq!(a.manifest_hash, b.manifest_hash);
}

#[test]
fn test_operator_a_to_b_continuity() {
    let operator_a = tempfile::tempdir().unwrap();
    let mut a_input = setup_fixture(operator_a.path());
    a_input.descriptor_output_path = operator_a
        .path()
        .join("worlds/default/recovery_descriptor.bin");
    let a_out = recover_world(a_input.clone()).unwrap();

    let operator_b = tempfile::tempdir().unwrap();
    fs::create_dir_all(operator_b.path().join("worlds/default")).unwrap();
    for file in [
        "world.wasm",
        "checkpoint_0.bin",
        "lineage.bin",
        "receipt_1.bin",
        "receipt_2.bin",
    ] {
        fs::copy(operator_a.path().join(file), operator_b.path().join(file)).unwrap();
    }
    fs::copy(
        operator_a
            .path()
            .join("worlds/default/recovery_descriptor.bin"),
        operator_b
            .path()
            .join("worlds/default/recovery_descriptor.bin"),
    )
    .unwrap();
    fs::copy(
        operator_a.path().join("worlds/default/manifest.bin"),
        operator_b.path().join("worlds/default/manifest.bin"),
    )
    .unwrap();

    let b_out = recover_world(OperatorRecoveryInput {
        package_path: operator_b.path().join("world.wasm"),
        checkpoint_path: operator_b.path().join("checkpoint_0.bin"),
        lineage_path: operator_b.path().join("lineage.bin"),
        receipt_paths: vec![
            operator_b.path().join("receipt_1.bin"),
            operator_b.path().join("receipt_2.bin"),
        ],
        descriptor_output_path: operator_b
            .path()
            .join("worlds/default/recovery_descriptor.bin"),
    })
    .unwrap();

    assert_eq!(a_out.descriptor_hash, b_out.descriptor_hash);
    assert_eq!(a_out.manifest_hash, b_out.manifest_hash);
    assert_eq!(
        a_out.report.recovered_state_root,
        b_out.report.recovered_state_root
    );
}
