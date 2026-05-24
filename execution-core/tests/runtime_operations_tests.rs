use execution_core::deployment::{
    bootstrap_runtime::RuntimeBootstrapEngine,
    bundle_builder::DeploymentBundleBuilder,
    image_builder::{ImageLayerProof, RuntimeImageBuilder},
    operational_store::{OperationalLedgerReader, OperationalLedgerWriter, OperationalStore},
    release_pipeline::ReleasePipeline,
    runtime_host::RuntimeHost,
    runtime_restore::RuntimeRestoreEngine,
    stdout_runtime::{OperationalProgressReport, RuntimeStatusEnvelope, StdoutRuntimeReporter},
    BootstrapManifest, RuntimeNodeLifecycle,
};

#[test]
fn test_runtime_bootstrap_execution() {
    let m = BootstrapManifest {
        genesis_hash: "g".into(),
        expected_package_hash: "p".into(),
    };
    let (_s, r) = RuntimeBootstrapEngine::execute(&m, "img");
    assert_eq!(r.deterministic_status, "DETERMINISTIC_OK");
}
#[test]
fn test_bootstrap_receipt_stability() {
    let m = BootstrapManifest {
        genesis_hash: "g".into(),
        expected_package_hash: "p".into(),
    };
    assert_eq!(
        RuntimeBootstrapEngine::execute(&m, "img").1,
        RuntimeBootstrapEngine::execute(&m, "img").1
    );
}
#[test]
fn test_bundle_generation_stability() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("a.tar.gz");
    let r1 =
        DeploymentBundleBuilder::generate(vec![("a".into(), b"1".to_vec())], p.to_str().unwrap()).1;
    let r2 =
        DeploymentBundleBuilder::generate(vec![("a".into(), b"1".to_vec())], p.to_str().unwrap()).1;
    assert_eq!(r1.bundle_hash, r2.bundle_hash);
}
#[test]
fn test_runtime_image_hash_stability() {
    let l = vec![
        ImageLayerProof {
            layer_name: "b".into(),
            layer_hash: "2".into(),
        },
        ImageLayerProof {
            layer_name: "a".into(),
            layer_hash: "1".into(),
        },
    ];
    assert_eq!(
        RuntimeImageBuilder::build(l.clone()).1.image_hash,
        RuntimeImageBuilder::build(l).1.image_hash
    );
}
#[test]
fn test_runtime_restoration_equivalence() {
    let (_s, r1) = RuntimeRestoreEngine::restore("b", "c", "l");
    let (_s2, r2) = RuntimeRestoreEngine::restore("b", "c", "l");
    assert_eq!(r1, r2);
}
#[test]
fn test_operational_ledger_append_only() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("ledger.bin");
    OperationalStore::append(&p, "a");
    OperationalStore::append(&p, "b");
    let e = OperationalLedgerReader::read(&p);
    assert_eq!(e, vec!["a".to_string(), "b".to_string()]);
    let cp = OperationalLedgerWriter::checkpoint(&e);
    assert_eq!(cp.height, 2);
}
#[test]
fn test_runtime_host_lifecycle() {
    let mut h = RuntimeHost::new();
    h.apply("BOOT", RuntimeNodeLifecycle::Bootstrapped);
    let r = h.apply("START", RuntimeNodeLifecycle::Active);
    assert_eq!(r.operation, "START");
}
#[test]
fn test_stdout_log_determinism() {
    let log1 = StdoutRuntimeReporter::report(
        vec![RuntimeStatusEnvelope {
            phase: "bootstrap".into(),
            continuity_root: "c".into(),
        }],
        vec![OperationalProgressReport {
            operation: "deploy".into(),
            status: "ok".into(),
        }],
    );
    let log2 = StdoutRuntimeReporter::report(
        vec![RuntimeStatusEnvelope {
            phase: "bootstrap".into(),
            continuity_root: "c".into(),
        }],
        vec![OperationalProgressReport {
            operation: "deploy".into(),
            status: "ok".into(),
        }],
    );
    assert_eq!(log1, log2);
}
#[test]
fn test_release_pipeline_equivalence() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("r.tar.gz");
    let (g, m) = ReleasePipeline::generate("1.0.0", p.to_str().unwrap());
    assert!(ReleasePipeline::verify(&m).verified);
    assert!(!g.bundle_hash.is_empty());
}
