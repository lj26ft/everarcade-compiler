use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_world_root() -> std::path::PathBuf {
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("everarcade_scheduler_world_{id}"));
    fs::create_dir_all(root.join("queue")).unwrap();
    root
}

#[test]
fn scheduler_status_reports_pending_events() {
    let world_root = temp_world_root();
    fs::write(
        world_root.join("queue/event-2.json"),
        r#"{"sequence":2,"source":"b","payload":"p2"}"#,
    )
    .unwrap();
    fs::write(
        world_root.join("queue/event-1.json"),
        r#"{"sequence":1,"source":"a","payload":"p1"}"#,
    )
    .unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .arg("scheduler-status")
        .arg("--world-root")
        .arg(&world_root)
        .output()
        .unwrap();

    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("scheduler_status=ok"));
    assert!(s.contains("latest_tick=0"));
    assert!(s.contains("pending_events=2"));
}

#[test]
fn scheduler_run_once_emits_receipt_and_advances_tick() {
    let world_root = temp_world_root();
    fs::write(
        world_root.join("queue/event-1.json"),
        r#"{"sequence":1,"source":"a","payload":"p1"}"#,
    )
    .unwrap();

    let run_out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .arg("scheduler-run-once")
        .arg("--world-root")
        .arg(&world_root)
        .output()
        .unwrap();
    assert!(run_out.status.success());

    let run_stdout = String::from_utf8_lossy(&run_out.stdout);
    assert!(run_stdout.contains("scheduler_run=ok"));
    assert!(run_stdout.contains("tick=1"));
    assert!(run_stdout.contains("events_processed=1"));
    assert!(run_stdout.contains("checkpoint_root="));
    assert!(run_stdout.contains("receipt="));

    let status_out = Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
        .arg("scheduler-status")
        .arg("--world-root")
        .arg(&world_root)
        .output()
        .unwrap();
    assert!(status_out.status.success());
    let status_stdout = String::from_utf8_lossy(&status_out.stdout);
    assert!(status_stdout.contains("latest_tick=1"));
}
