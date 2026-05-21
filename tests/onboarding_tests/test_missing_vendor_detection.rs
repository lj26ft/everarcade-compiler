use std::{path::Path, process::Command};

#[test]
fn test_missing_vendor_detection() {
    if !Path::new("vendor").exists() {
        return;
    }
    let output = Command::new("bash")
        .arg("-lc")
        .arg("mv vendor vendor.bak && ./scripts/doctor_quick.sh; code=$?; mv vendor.bak vendor; exit $code")
        .output()
        .expect("run doctor quick");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("vendor/ missing"));
}
