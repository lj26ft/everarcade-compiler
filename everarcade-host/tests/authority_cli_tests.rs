use std::process::Command;

use tempfile::tempdir;

fn node(v: u8) -> String {
    hex::encode([v; 32])
}

#[test]
fn authority_status_verify_handoff_commands() {
    let t = tempdir().unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "authority-status",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(status.status.success());
    assert!(String::from_utf8_lossy(&status.stdout).contains("authority_status=ok"));

    let verify = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "authority-verify",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(verify.status.success());
    assert!(String::from_utf8_lossy(&verify.stdout).contains("authority_verify=ok"));

    let from = node(0);
    let to = node(7);
    let handoff = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "authority-handoff",
            "--world-root",
            t.path().to_str().unwrap(),
            "--from",
            &from,
            "--to",
            &to,
        ])
        .output()
        .unwrap();
    assert!(handoff.status.success());
    let out = String::from_utf8_lossy(&handoff.stdout);
    assert!(out.contains("authority_handoff=ok"));
    assert!(out.contains("new_epoch=1"));
}

#[test]
fn lease_status_verify_renew_commands() {
    let t = tempdir().unwrap();
    let status = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["lease-status", "--world-root", t.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(status.status.success());
    assert!(String::from_utf8_lossy(&status.stdout).contains("lease_status=ok"));
    let verify = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["lease-verify", "--world-root", t.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(verify.status.success());
    assert!(String::from_utf8_lossy(&verify.stdout).contains("lease_verify=ok"));
    let renew = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args(["lease-renew", "--world-root", t.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(renew.status.success());
    assert!(String::from_utf8_lossy(&renew.stdout).contains("lease_renew=ok"));
}
