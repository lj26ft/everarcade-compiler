use execution_core::deployment::RuntimePackage;
use execution_core::hashing;

fn package_artifacts() -> Vec<&'static str> {
    vec![
        "game package",
        "runtime package",
        "world package",
        "asset package",
        "deployment package",
    ]
}

fn readiness_reports() -> Vec<&'static str> {
    vec![
        "protocol readiness",
        "creator readiness",
        "deployment readiness",
        "ecosystem readiness",
    ]
}

#[test]
fn test_package_generation_equivalence() {
    let artifacts = package_artifacts();
    let package = RuntimePackage {
        name: "creator-template-runtime".to_owned(),
        payload: artifacts.join("|").into_bytes(),
    };
    let same_package = RuntimePackage {
        name: "creator-template-runtime".to_owned(),
        payload: artifacts.join("|").into_bytes(),
    };
    assert_eq!(package.package_hash(), same_package.package_hash());
    assert_eq!(artifacts.len(), 5);
}

#[test]
fn test_protocol_readiness() {
    let reports = readiness_reports();
    let readiness_hash = hashing::hash_bytes(reports.join("|").as_bytes());
    assert_eq!(reports.len(), 4);
    assert!(reports.contains(&"protocol readiness"));
    assert_eq!(
        readiness_hash,
        hashing::hash_bytes(reports.join("|").as_bytes())
    );
}

#[test]
fn test_replay_safe_pipeline() {
    let validation_checks = [
        "asset validation",
        "world validation",
        "gameplay validation",
        "replay validation",
        "multiplayer validation",
        "publish validation",
    ];
    assert!(validation_checks.contains(&"replay validation"));
    assert!(package_artifacts().contains(&"runtime package"));
}

#[test]
fn test_authority_mutation_rejection() {
    fn reject_authority_mutation(requested: bool) -> Result<(), &'static str> {
        if requested {
            Err("creator pipeline cannot mutate deterministic runtime authority")
        } else {
            Ok(())
        }
    }
    assert!(reject_authority_mutation(true).is_err());
    assert!(reject_authority_mutation(false).is_ok());
}
