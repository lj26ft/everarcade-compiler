use std::{fs, path::Path, process::Command};

#[test]
fn test_replay_artifact_generation() {
    let status = Command::new("cargo")
        .args(["run", "-p", "everarcade-cli", "--", "replay-world"])
        .status()
        .expect("failed to run replay-world");
    assert!(status.success());
    assert!(Path::new(".everarcade-dev/replay.log").exists());
    let body = fs::read_to_string(".everarcade-dev/replay.log").expect("read replay.log");
    assert!(body.contains("convergence=verified"));
}
