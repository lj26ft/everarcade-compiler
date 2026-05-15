use std::process::Command;

use everarcade_host::fixture::generate_fixture_to_path;

fn temp_path() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("everarcade-live-test-{}", n));
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn deploy_proof_emits_end_to_end_chain() {
    let root = temp_path();
    let package = root.join("civilization_package.bin");
    generate_fixture_to_path(&package).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "deploy-proof",
            "--package",
            package.to_str().unwrap(),
            "--state",
            root.to_str().unwrap(),
            "--profile",
            "live",
            "--node",
            "evernode-op-a",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("package=ok"));
    assert!(stdout.contains("execute=ok"));
    assert!(stdout.contains("receipt="));
    assert!(stdout.contains("checkpoint="));
    assert!(stdout.contains("distributed-receipt=ok"));
    assert!(stdout.contains("xrpl-anchor="));
    assert!(stdout.contains("ipfs-manifest="));
}
