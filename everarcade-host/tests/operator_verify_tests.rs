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

use everarcade_host::fixture::generate_fixture_to_path;
use std::process::Command;
#[test]
fn verify_fails_on_corrupt_receipt() {
    let d = temp_path();
    assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["init", "--state", d.as_path().to_str().unwrap()])
        .status()
        .unwrap()
        .success());
    assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "run",
            "--package",
            fixture_path().to_str().unwrap(),
            "--state",
            d.as_path().to_str().unwrap()
        ])
        .status()
        .unwrap()
        .success());
    let p = std::fs::read_dir(d.as_path().join("receipts"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    std::fs::write(&p, b"bad").unwrap();
    assert!(!Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["verify", "--state", d.as_path().to_str().unwrap()])
        .status()
        .unwrap()
        .success());
}
