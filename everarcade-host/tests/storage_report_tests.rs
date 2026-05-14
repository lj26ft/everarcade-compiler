use everarcade_host::{
    config::HostConfig, fixture::generate_fixture_to_path, run_package_once,
    state_folder::storage_report::storage_report,
};
use std::{fs, time::{SystemTime, UNIX_EPOCH}};

#[test]
fn storage_report_counts_artifacts() {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("everarcade-storage-report-{unique}"));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("mkdir");
    let state = root.join("state");
    let fixture = root.join("fixture.bin");
    generate_fixture_to_path(&fixture).expect("fixture");

    let _ = run_package_once(HostConfig::new(fixture, state.clone())).expect("run");
    let report = storage_report(&state).expect("report");

    assert!(report.receipt_count >= 1);
    assert!(report.checkpoint_count >= 1);
    assert!(report.anchor_count >= 1);
    assert!(report.total_bytes > 0);
}

