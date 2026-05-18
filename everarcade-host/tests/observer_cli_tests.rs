use std::{fs, process::Command};

use execution_core::{
    federation::node::FederationNodeId,
    sync::{cursor::SyncCursor, observer::ObserverState, persistence::save_observer_state},
};
use tempfile::tempdir;

#[test]
fn observer_status_resume_verify_commands() {
    let t = tempdir().unwrap();
    let state = ObserverState {
        world_id: "world".into(),
        operator: FederationNodeId::new([3; 32]),
        current_cursor: SyncCursor {
            latest_sequence: 0,
            latest_execution_id: [1; 32],
            latest_checkpoint_root: [2; 32],
            latest_manifest_hash: [3; 32],
            latest_lineage_hash: [4; 32],
        },
        highest_verified_sequence: 0,
        latest_checkpoint_root: [2; 32],
        synchronized: true,
    };
    save_observer_state(t.path(), &state).unwrap();

    let s = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "observer-status",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let out = String::from_utf8_lossy(&s.stdout);
    assert!(out.contains("observer_status=ok"));

    let r = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "observer-resume",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&r.stdout).contains("observer_resume=ok"));

    let v = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "observer-verify",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&v.stdout).contains("observer_verify=ok"));

    let ts = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "topology-status",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&ts.stdout).contains("topology_status=ok"));

    let tc = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "topology-convergence",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&tc.stdout).contains("topology_convergence=ok"));

    let tp = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .args([
            "topology-propagation",
            "--world-root",
            t.path().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&tp.stdout).contains("topology_propagation=ok"));

    let _ = fs::remove_dir_all(t.path());
}
