use std::process::Command;

#[test]
fn test_local_federation_boot() {
    let status = Command::new("cargo")
        .args(["run", "-p", "everarcade-cli", "--", "run-local-federation"])
        .status()
        .expect("failed to run local federation");
    assert!(status.success());
}
