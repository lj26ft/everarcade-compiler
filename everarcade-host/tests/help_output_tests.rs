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
    assert!(s.contains("import-bundle --bundle <bundle_root> --world-root <path>"));
    assert!(s.contains("verify-bundle --bundle <bundle_root>"));
    assert!(s.contains("export-bundle --out <bundle_root>"));
    assert!(s.contains("scheduler-run-once --world-root <path>"));
    assert!(s.contains("scheduler-status --world-root <path>"));
}
