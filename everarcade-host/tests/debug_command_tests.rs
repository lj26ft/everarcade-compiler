use std::process::Command;
#[test]
fn debug_command_outputs_operator_fields() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state");
    assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["init", "--state", state.to_str().unwrap()])
        .status()
        .unwrap()
        .success());
    let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["debug", "--state", state.to_str().unwrap()])
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("version="));
    assert!(s.contains("state_exists=true"));
    assert!(s.contains("anchor_queue_count="));
}
