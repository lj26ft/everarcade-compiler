use std::{path::Path, process::Command};

#[test]
fn test_reset_flow() {
    let status = Command::new("cargo")
        .args(["run", "-p", "everarcade-cli", "--", "reset"])
        .status()
        .expect("run reset");
    assert!(status.success());
    assert!(Path::new(".everarcade-dev/replay.log").exists());
}
