use std::process::Command;
use tempfile::tempdir;

#[test]
fn consensus_commands() {
    let t = tempdir().unwrap();
    for (cmd, needle) in [
        ("consensus-status", "consensus_status=ok"),
        ("consensus-verify", "consensus_verify=ok"),
        ("register-proposal", "register_proposal=ok"),
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
            .args([cmd, "--world-root", t.path().to_str().unwrap()])
            .output()
            .unwrap();
        assert!(out.status.success());
        assert!(String::from_utf8_lossy(&out.stdout).contains(needle));
    }
}
