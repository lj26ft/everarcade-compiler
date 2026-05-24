mod support;

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
use support::runtime_validation::{
    assert_bundle_equivalence, assert_operational_ledger_stability, assert_receipt_equivalence,
    assert_restore_equivalence, assert_stdout_determinism,
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
fn test_full_runtime_operations_pipeline() {
    let m = BootstrapManifest {
        genesis_hash: "g".into(),
        expected_package_hash: "p".into(),
    };
    let (_boot, boot_r1) = RuntimeBootstrapEngine::execute(&m, "img");
    let (_boot2, boot_r2) = RuntimeBootstrapEngine::execute(&m, "img");
    assert_receipt_equivalence(&boot_r1, &boot_r2);

    let d = tempfile::tempdir().unwrap();
    let bundle_path = d.path().join("bundle.tar.gz");
    let (_a1, bundle_r1) = DeploymentBundleBuilder::generate(
        vec![("a".into(), b"1".to_vec())],
        bundle_path.to_str().unwrap(),
    );
    let (_a2, bundle_r2) = DeploymentBundleBuilder::generate(
        vec![("a".into(), b"1".to_vec())],
        bundle_path.to_str().unwrap(),
    );
    assert_bundle_equivalence(&bundle_r1, &bundle_r2);

    let ledger = d.path().join("ledger.bin");
    for op in [
        "BOOTSTRAP",
        "BUNDLE",
        "CHECKPOINT",
        "ARCHIVE",
        "RESTORE",
        "VERIFY",
        "SHUTDOWN",
    ] {
        OperationalStore::append(&ledger, op);
    }
    let entries = OperationalLedgerReader::read(&ledger);
    let cp1 = OperationalLedgerWriter::checkpoint(&entries);
    let cp2 = OperationalLedgerWriter::checkpoint(&entries);
    assert_operational_ledger_stability(&cp1, &cp2);

    let (_st1, rr1) =
        RuntimeRestoreEngine::restore(&bundle_r1.bundle_hash, "checkpoint", &cp1.root);
    let (_st2, rr2) =
        RuntimeRestoreEngine::restore(&bundle_r1.bundle_hash, "checkpoint", &cp1.root);
    assert_restore_equivalence(&rr1, &rr2);

    let log1 = StdoutRuntimeReporter::report(
        vec![RuntimeStatusEnvelope {
            phase: "verify".into(),
            continuity_root: rr1.continuity_root.clone(),
        }],
        vec![OperationalProgressReport {
            operation: "SHUTDOWN".into(),
            status: "OK".into(),
        }],
    );
    let log2 = StdoutRuntimeReporter::report(
        vec![RuntimeStatusEnvelope {
            phase: "verify".into(),
            continuity_root: rr1.continuity_root,
        }],
        vec![OperationalProgressReport {
            operation: "SHUTDOWN".into(),
            status: "OK".into(),
        }],
    );
    assert_stdout_determinism(&log1, &log2);
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
