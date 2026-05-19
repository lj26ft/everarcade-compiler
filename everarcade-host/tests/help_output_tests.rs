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
    assert!(s.contains("topology-propagation --world-root <path>"));
    assert!(s.contains("topology-convergence --world-root <path>"));
    assert!(s.contains("topology-status --world-root <path>"));
    assert!(s.contains("authority-status --world-root <path>"));
    assert!(s.contains("authority-verify --world-root <path>"));
    assert!(s.contains("authority-handoff --world-root <path> --from <node-id> --to <node-id>"));
    assert!(s.contains("lease-status --world-root <path>"));
    assert!(s.contains("lease-verify --world-root <path>"));
    assert!(s.contains("lease-renew --world-root <path>"));
    assert!(s.contains("detect-fork --world-root <path>"));
    assert!(s.contains("divergence-verify --world-root <path>"));
    assert!(s.contains("divergence-status --world-root <path>"));
    assert!(s.contains("reconciliation-status --world-root <path>"));
    assert!(s.contains("reconciliation-verify --world-root <path>"));
    assert!(s.contains("quarantine-fork --world-root <path>"));
}
