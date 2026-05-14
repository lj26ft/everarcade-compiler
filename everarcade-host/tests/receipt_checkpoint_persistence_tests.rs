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
use everarcade_host::{run_package_once, HostConfig};
#[test]
fn receipt_checkpoint_written() {
    let d = temp_path();
    let _ = run_package_once(HostConfig::new(
        &format!(
            "{}/tests/fixtures/civilization_package.bin",
            env!("CARGO_MANIFEST_DIR")
        ),
        d.as_path(),
    ))
    .unwrap();
    assert!(d.as_path().join("receipts").exists());
    assert!(d.as_path().join("checkpoints").exists());
}
