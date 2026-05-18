use std::process::Command;

#[test]
fn sync_commands_are_exposed() {
    let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("sync-advertise --world-root <path>"));
    assert!(s.contains("sync-verify --bundle <path>"));
    assert!(s.contains("sync-pull --world-root <path> --start-sequence <n> --end-sequence <n>"));
    assert!(s.contains("observer-status --world-root <path>"));
    assert!(s.contains("observer-resume --world-root <path>"));
    assert!(s.contains("observer-verify --world-root <path>"));
}
