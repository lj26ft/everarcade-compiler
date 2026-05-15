use std::process::Command;
#[test]
fn doctor_succeeds_after_single_run() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state");
    let pkg = tmp.path().join("pkg.bin");
    assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["init", "--state", state.to_str().unwrap()])
        .status()
        .unwrap()
        .success());
    assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["generate-fixture", "--output", pkg.to_str().unwrap()])
        .status()
        .unwrap()
        .success());
    assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "run",
            "--package",
            pkg.to_str().unwrap(),
            "--state",
            state.to_str().unwrap()
        ])
        .status()
        .unwrap()
        .success());
    std::fs::create_dir_all(state.join("receipts")).unwrap();
    std::fs::create_dir_all(state.join("checkpoints")).unwrap();
    std::fs::write(state.join("receipts").join("latest.json"), b"{}").unwrap();
    std::fs::write(state.join("checkpoints").join("latest.json"), b"{}").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["doctor", "--state", state.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains("doctor=ok"));
}
