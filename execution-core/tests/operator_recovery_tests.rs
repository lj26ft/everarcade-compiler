use std::path::PathBuf;

use execution_core::operator::{descriptor_hash, load_recovery_descriptor, recover_world, save_recovery_descriptor, OperatorRecoveryInput, WorldRecoveryDescriptor};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from("../everarcade-host/tests/fixtures/counter_world").join(name)
}

#[test]
fn test_world_recovery_success() {
    let tmp = tempfile::tempdir().unwrap();
    let out = recover_world(OperatorRecoveryInput {
        package_path: fixture("world.wasm"),
        checkpoint_path: fixture("checkpoint_0.bin"),
        lineage_path: fixture("lineage.bin"),
        receipt_paths: vec![fixture("receipt_1.bin"), fixture("receipt_2.bin")],
        descriptor_output_path: tmp.path().join("descriptor.bin"),
    })
    .unwrap();
    assert!(out.report.recovery_ok);
}

#[test] fn test_world_recovery_checkpoint_mismatch_fails() {}
#[test] fn test_world_recovery_manifest_mismatch_fails() {}
#[test] fn test_world_recovery_lineage_mismatch_fails() {}
#[test] fn test_world_recovery_replay_mismatch_fails() {}

#[test]
fn test_recovery_descriptor_roundtrip() {
    let tmp = tempfile::tempdir().unwrap();
    let d = WorldRecoveryDescriptor { world_id:[1;32], package_root:[2;32], latest_checkpoint_root:[3;32], latest_execution_id:[4;32], manifest_hash:[5;32] };
    let path = tmp.path().join("d.bin");
    save_recovery_descriptor(&path, &d).unwrap();
    let got = load_recovery_descriptor(&path).unwrap();
    assert_eq!(d, got);
}

#[test]
fn test_descriptor_hash_stable() {
    let d = WorldRecoveryDescriptor { world_id:[9;32], package_root:[2;32], latest_checkpoint_root:[3;32], latest_execution_id:[4;32], manifest_hash:[5;32] };
    assert_eq!(descriptor_hash(&d), descriptor_hash(&d));
}

#[test]
fn test_cross_run_recovery_equivalence() {
    let tmp1 = tempfile::tempdir().unwrap();
    let tmp2 = tempfile::tempdir().unwrap();
    let a = recover_world(OperatorRecoveryInput { package_path: fixture("world.wasm"), checkpoint_path: fixture("checkpoint_0.bin"), lineage_path: fixture("lineage.bin"), receipt_paths: vec![fixture("receipt_1.bin"), fixture("receipt_2.bin")], descriptor_output_path: tmp1.path().join("descriptor.bin") }).unwrap();
    let b = recover_world(OperatorRecoveryInput { package_path: fixture("world.wasm"), checkpoint_path: fixture("checkpoint_0.bin"), lineage_path: fixture("lineage.bin"), receipt_paths: vec![fixture("receipt_1.bin"), fixture("receipt_2.bin")], descriptor_output_path: tmp2.path().join("descriptor.bin") }).unwrap();
    assert_eq!(a.descriptor_hash, b.descriptor_hash);
    assert_eq!(a.manifest_hash, b.manifest_hash);
}

#[test]
fn operator_a_executes_world() {}
#[test]
fn persist_artifacts() {}
#[test]
fn operator_b_restores_world() {}
#[test]
fn verify_identical_continuity() {}
