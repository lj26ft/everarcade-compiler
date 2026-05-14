use std::{path::PathBuf, process::Command};

#[test]
fn stress_script_exists_and_is_executable() {
    let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("scripts/linux_vm_stress.sh");

    let output = Command::new("bash")
        .arg("-n")
        .arg(&script_path)
        .output()
        .expect("bash -n");
    assert!(
        output.status.success(),
        "bash -n failed for {}",
        script_path.display()
    );
}
