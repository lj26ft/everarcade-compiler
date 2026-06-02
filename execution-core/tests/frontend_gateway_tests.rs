use serde_json::Value;
use std::fs;

fn repo_file(path: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let full_path = root.join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|err| panic!("missing {}: {err}", full_path.display()))
}

#[test]
fn test_doctor_endpoint() {
    let routes: Value = serde_json::from_str(&repo_file("frontend-gateway/routes.json")).unwrap();
    assert_eq!(
        routes["endpoints"]["/doctor"]["command"],
        "everarcade doctor --json"
    );
}

#[test]
fn test_status_endpoint() {
    let routes: Value = serde_json::from_str(&repo_file("frontend-gateway/routes.json")).unwrap();
    assert_eq!(
        routes["endpoints"]["/status"]["command"],
        "everarcade status --json"
    );
}

#[test]
fn test_package_endpoint() {
    let routes: Value = serde_json::from_str(&repo_file("frontend-gateway/routes.json")).unwrap();
    assert_eq!(
        routes["endpoints"]["/package"]["command"],
        "everarcade package --json"
    );
}

#[test]
fn test_rehearse_endpoint() {
    let routes: Value = serde_json::from_str(&repo_file("frontend-gateway/routes.json")).unwrap();
    assert_eq!(
        routes["endpoints"]["/rehearse"]["command"],
        "everarcade rehearse --json"
    );
}

#[test]
fn test_validate_endpoint() {
    let routes: Value = serde_json::from_str(&repo_file("frontend-gateway/routes.json")).unwrap();
    assert_eq!(
        routes["endpoints"]["/validate"]["command"],
        "everarcade validate --profile quick --json"
    );
}

#[test]
fn test_deploy_dry_run_endpoint() {
    let routes: Value = serde_json::from_str(&repo_file("frontend-gateway/routes.json")).unwrap();
    assert_eq!(
        routes["endpoints"]["/deploy"]["command"],
        "everarcade deploy --dry-run --json"
    );
}

#[test]
fn test_json_contracts() {
    let contracts = repo_file("docs/frontend/json_contracts.md");
    for contract in [
        "DoctorResult",
        "StatusResult",
        "PackageResult",
        "ValidationResult",
        "DeploymentResult",
    ] {
        assert!(contracts.contains(contract), "contract missing: {contract}");
    }
}
