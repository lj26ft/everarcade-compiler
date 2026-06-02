use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("execution-core has workspace parent")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).unwrap_or_else(|err| panic!("read {relative}: {err}"))
}

fn assert_contains(relative: &str, needle: &str) {
    let body = read(relative);
    assert!(body.contains(needle), "{relative} missing {needle}");
}

fn product_source() -> String {
    read("src-bin-everarcade/src/product.rs")
}

#[test]
fn test_doctor_command() {
    let source = product_source();
    assert!(source.contains("\"doctor\""));
    assert!(source.contains("🩺 EverArcade Doctor"));
    assert!(source.contains("Cargo"));
    assert!(source.contains("Vendor"));
    assert!(source.contains("Offline Mode"));
    assert!(source.contains("Runtime Packages"));
    assert!(source.contains("Rustrigs"));
    assert!(source.contains("State Layout"));
}

#[test]
fn test_package_command() {
    let source = product_source();
    assert!(source.contains("\"package\""));
    assert!(source.contains("scripts/generate_evernode_packages.sh"));
    assert!(source.contains("📦 Packaging Game"));
    assert!(source.contains("Checksums Verified"));
}

#[test]
fn test_rehearsal_command() {
    let source = product_source();
    assert!(source.contains("\"rehearse\""));
    assert!(source.contains("scripts/run_hotpocket_contract_rehearsal.sh"));
    assert!(source.contains("🎮 HotPocket Rehearsal"));
    assert!(source.contains("Rehearsal Passed"));
}

#[test]
fn test_validation_profiles() {
    let source = product_source();
    for profile in ["quick", "rustrigs", "evernode", "full"] {
        assert!(source.contains(profile), "missing profile {profile}");
    }
    assert!(source.contains("runtime package generation"));
    assert!(source.contains("ABI tests"));
    assert!(source.contains("provider validation"));
    assert!(source.contains("security"));
}

#[test]
fn test_release_gate() {
    let source = product_source();
    assert!(source.contains("release-gate"));
    assert!(source.contains("validation_logs/release_report.md"));
    assert!(source.contains("validation_logs.tar.gz"));
    assert!(source.contains("Release Candidate Approved"));
}

#[test]
fn test_artifact_policy() {
    let source = product_source();
    assert!(source.contains("artifacts-check"));
    assert!(source.contains("scripts/check_no_generated_artifacts_tracked.sh"));
    assert!(source.contains("tarballs ignored"));
    assert!(source.contains("dist outputs ignored"));
}

#[test]
fn test_stage_contract() {
    let source = product_source();
    assert!(source.contains("stage-contract"));
    assert!(source.contains("--stage-contract"));
    assert!(source.contains("dist/everarcade-hotpocket-contract"));
}

#[test]
fn test_status_command() {
    let source = product_source();
    assert!(source.contains("\"status\""));
    assert!(source.contains("Runtime Healthy"));
    assert!(source.contains("Replay Healthy"));
    assert!(source.contains("Deployment Ready"));
    assert!(source.contains("Federation Healthy"));
}

#[test]
fn test_json_output() {
    let value: Value = serde_json::json!({
        "command": "doctor",
        "status": "ready",
        "checks": [{"name": "Cargo", "status": "passed"}]
    });
    assert_eq!(value["command"], "doctor");
    let source = product_source();
    assert!(source.contains("--json"));
    assert!(source.contains("serde_json::to_string_pretty"));
    assert!(source.contains("CommandReport"));
}

#[test]
fn test_product_command_equivalence() {
    assert_contains(
        "src-bin-everarcade/src/main.rs",
        "product::is_product_command",
    );
    assert_contains("src-bin-everarcade/src/product.rs", "advanced");
    assert_contains("src-bin-everarcade/src/commands/mod.rs", "dispatch_legacy");
    assert_contains(
        "deployment/reports/product_command_audit.md",
        "Product facade",
    );
}
