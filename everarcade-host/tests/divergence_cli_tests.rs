use std::process::Command;
use tempfile::tempdir;

#[test]
fn divergence_status_verify_detect_commands() {
    let t = tempdir().unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "divergence-status",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(status.status.success());
    assert!(String::from_utf8_lossy(&status.stdout).contains("divergence_status=ok"));

    let verify = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "divergence-verify",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(verify.status.success());
    assert!(String::from_utf8_lossy(&verify.stdout).contains("divergence_verify=ok"));

    let detect = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["detect-fork", "--world-root", t.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(detect.status.success());
    assert!(String::from_utf8_lossy(&detect.stdout).contains("detect_fork=ok"));
}
