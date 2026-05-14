fn temp_path() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("everarcade-test-{}", n));
    std::fs::create_dir_all(&p).unwrap();
    p
}
fn fixture_path() -> std::path::PathBuf {
    let path = temp_path().join("civilization_package.bin");
    generate_fixture_to_path(&path).unwrap();
    path
}

use everarcade_host::{fixture::generate_fixture_to_path, run_package_once, HostConfig};
#[test]
fn receipt_checkpoint_written() {
    let d = temp_path();
    let _ = run_package_once(HostConfig::new(
        fixture_path().to_str().unwrap(),
        d.as_path(),
    ))
    .unwrap();
    assert!(d.as_path().join("receipts").exists());
    assert!(d.as_path().join("checkpoints").exists());
}
