use std::process::Command;

#[test]
fn test_doctor_output() {
    let output = Command::new("./scripts/doctor_quick.sh")
        .output()
        .expect("run doctor quick");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PASS") || stdout.contains("FAIL"));
}
