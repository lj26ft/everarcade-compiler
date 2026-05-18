fn temp_path() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("everarcade-test-{}", n));
    std::fs::create_dir_all(&p).unwrap();
    p
}
fn fixture_path() -> std::path::PathBuf {
    let path = temp_path().join("civilization_package.bin");
    generate_fixture_to_path(&path).unwrap();
    path
}

use everarcade_host::{
    fixture::generate_fixture_to_path, package_loader::load_package, run_package_once, HostConfig,
};
use execution_core::vm::{validate_vm_receipt, VmExecutionInput};

#[test]
fn fixture_round_trip() {
    let pkg = load_package(fixture_path().as_path()).unwrap();
    let _input = VmExecutionInput {
        package_manifest_root: pkg.execution_root,
        civilization_root: pkg.execution_root,
        pre_state_root: pkg.replay_root,
        prior_replay_root_value: pkg.replay_root,
        checkpoint_root: pkg.checkpoint_root,
        payload_root: pkg.proof_root,
    };
    let dir = temp_path();
    let out = run_package_once(HostConfig::new(
        fixture_path().to_str().unwrap(),
        dir.as_path(),
    ))
    .unwrap();
    assert!(validate_vm_receipt(&out.receipt));
}
