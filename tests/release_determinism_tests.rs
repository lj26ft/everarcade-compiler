use std::process::Command;

#[test]
fn release_mode_requires_source_date_epoch() {
    let out = Command::new("bash")
        .arg("scripts/build_runtime_release.sh")
        .arg("--release")
        .env_remove("SOURCE_DATE_EPOCH")
        .output()
        .expect("script should run");
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("SOURCE_DATE_EPOCH is required in release mode"));
}
