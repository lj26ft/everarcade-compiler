use std::process::Command;

#[test]
fn stress_script_exists_and_is_executable() {
    let output = Command::new("bash")
        .arg("-n")
        .arg("scripts/linux_vm_stress.sh")
        .output()
        .expect("bash -n");
    assert!(output.status.success());
}
