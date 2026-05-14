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
use everarcade_host::{package_loader::load_package, run_package_once, HostConfig};
use execution_core::vm::{validate_vm_receipt, VmExecutionInput};

#[test]
fn fixture_round_trip() {
    let pkg = load_package(std::path::Path::new(&format!(
        "{}/tests/fixtures/civilization_package.bin",
        env!("CARGO_MANIFEST_DIR")
    )))
    .unwrap();
    let _input = VmExecutionInput {
        package_manifest_root: pkg.execution_root,
        civilization_root: pkg.execution_root,
        replay_root: pkg.replay_root,
        checkpoint_root: pkg.checkpoint_root,
        payload_root: pkg.proof_root,
    };
    let dir = temp_path();
    let out = run_package_once(HostConfig::new(
        &format!(
            "{}/tests/fixtures/civilization_package.bin",
            env!("CARGO_MANIFEST_DIR")
        ),
        dir.as_path(),
    ))
    .unwrap();
    assert!(validate_vm_receipt(&out.receipt));
}
