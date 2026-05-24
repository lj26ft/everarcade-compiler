use execution_core::federation::simulation::{fixture_catalog, run_fixture, topology_manifest, DivergenceType};

#[test]
fn test_multi_node_replay_equivalence() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "5-node federation").unwrap();
    let report = run_fixture(&fixture, None);
    assert!(report.validations.iter().all(|v| v.equivalent));
}

#[test]
fn test_archive_sync_equivalence() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "3-node federation").unwrap();
    let report = run_fixture(&fixture, None);
    let first = &report.diagnostics[0].archive_root;
    assert!(report.diagnostics.iter().all(|d| &d.archive_root == first));
}

#[test]
fn test_federation_recovery_equivalence() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "restored federation").unwrap();
    let report = run_fixture(&fixture, None);
    let root = report.validations.first().unwrap().replay_root.clone();
    assert_eq!(report.validations.last().unwrap().replay_root, root);
}

#[test]
fn test_divergence_detection() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "2-node federation").unwrap();
    let report = run_fixture(&fixture, Some(DivergenceType::ReceiptMismatch));
    assert!(report.diagnostics.iter().any(|d| d.divergence_detected));
}

#[test]
fn test_topology_manifest_stability() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "3-node federation").unwrap();
    let a = topology_manifest(&fixture);
    let b = topology_manifest(&fixture);
    assert_eq!(a, b);
}

#[test]
fn test_restoration_replay_equivalence() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "archive recovery federation").unwrap();
    let report = run_fixture(&fixture, None);
    assert!(report.validations.iter().all(|v| v.replay_root == report.validations[0].replay_root));
}

#[test]
fn test_partition_recovery_equivalence() {
    let fixture = fixture_catalog().into_iter().find(|f| f.federation_id == "partitioned federation").unwrap();
    let report = run_fixture(&fixture, None);
    assert!(report.validations.iter().all(|v| v.equivalent));
}
