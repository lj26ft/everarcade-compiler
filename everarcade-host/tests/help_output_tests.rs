use std::process::Command;
#[test]
fn help_exits_zero_and_contains_commands() {
    let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("EverArcade Host Operator"));
    assert!(s.contains("debug --state <path>"));
    assert!(s.contains("doctor --state <path>"));
    assert!(s.contains("lineage-verify --lineage <path>"));
}
