use std::process::Command;
use tempfile::tempdir;

#[test]
fn coordination_commands() {
    let t = tempdir().unwrap();
    for (cmd, needle) in [
        ("coordination-status", "coordination_status=ok"),
        ("coordination-verify", "coordination_verify=ok"),
        (
            "register-coordination-session",
            "register_coordination_session=ok",
        ),
        ("envelope-status", "envelope_status=ok"),
        ("envelope-verify", "envelope_verify=ok"),
        ("register-envelope-message", "register_envelope_message=ok"),
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
            .args([cmd, "--world-root", t.path().to_str().unwrap()])
            .output()
            .unwrap();
        assert!(out.status.success());
        assert!(String::from_utf8_lossy(&out.stdout).contains(needle));
    }
}
