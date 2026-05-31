use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("execution-core has workspace parent")
        .to_path_buf()
}

fn runtime_dir() -> PathBuf {
    root().join("deployment/evernode/runtime")
}

fn run_build_script() {
    let status = Command::new("bash")
        .arg("scripts/build_evernode_packages.sh")
        .current_dir(root())
        .status()
        .expect("run EverNode package builder");
    assert!(status.success(), "EverNode package builder failed");
}

fn sha256(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum available");
    assert!(output.status.success(), "sha256sum failed for {path:?}");
    String::from_utf8(output.stdout)
        .expect("sha256 output is utf8")
        .split_whitespace()
        .next()
        .expect("sha256 output includes digest")
        .to_owned()
}

#[test]
fn test_evernode_packages_are_generated_and_validated() {
    run_build_script();

    let expected = [
        "arena-vanguard-runtime",
        "arena-vanguard-world",
        "arena-vanguard-deployment",
    ];
    let checksums = fs::read_to_string(runtime_dir().join("packages.sha256"))
        .expect("packages.sha256 generated");

    for package in expected {
        let tarball = runtime_dir().join(format!("{package}.tar.gz"));
        let signature = runtime_dir().join(format!("{package}.sig"));
        assert!(tarball.is_file(), "missing generated tarball {tarball:?}");
        assert!(
            signature.is_file(),
            "missing generated signature {signature:?}"
        );

        let digest = sha256(&tarball);
        assert!(
            checksums.contains(&format!("{digest}  {package}.tar.gz")),
            "packages.sha256 missing digest for {package}"
        );

        let signature_body = fs::read_to_string(&signature).expect("read generated signature");
        assert_eq!(
            signature_body.trim(),
            format!("evernode-offline-signature-v1 {package} {digest}"),
            "signature should bind package name to generated tarball digest"
        );
    }
}

#[test]
fn test_evernode_generated_binaries_are_not_source_controlled() {
    let output = Command::new("git")
        .args([
            "ls-files",
            "deployment/evernode/runtime/*.tar.gz",
            "deployment/evernode/runtime/*.sig",
        ])
        .current_dir(root())
        .output()
        .expect("git ls-files runs");
    assert!(output.status.success(), "git ls-files failed");
    assert!(
        output.stdout.is_empty(),
        "generated EverNode tarballs/signatures must not be tracked: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn test_evernode_package_builder_is_reproducible() {
    run_build_script();
    let first = fs::read_to_string(runtime_dir().join("packages.sha256"))
        .expect("read first generated package checksum manifest");
    run_build_script();
    let second = fs::read_to_string(runtime_dir().join("packages.sha256"))
        .expect("read second generated package checksum manifest");
    assert_eq!(first, second, "EverNode package generation must be stable");
}
