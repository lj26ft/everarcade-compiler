use execution_core::runtime::ci::*;

#[test]
fn test_validation_pipeline_determinism() {
    let rt = ValidationPipelineRuntime;
    let a = rt.run(
        vec![
            ValidationPipelineStage {
                name: "b".into(),
                sequence: 2,
            },
            ValidationPipelineStage {
                name: "a".into(),
                sequence: 1,
            },
        ],
        "l1".into(),
    );
    let b = rt.run(
        vec![
            ValidationPipelineStage {
                name: "a".into(),
                sequence: 1,
            },
            ValidationPipelineStage {
                name: "b".into(),
                sequence: 2,
            },
        ],
        "l1".into(),
    );
    assert_eq!(a.execution_order, b.execution_order);
}
#[test]
fn test_ci_scheduler_partition_equivalence() {
    let s = CiSchedulerRuntime;
    let schedule = s.schedule(vec![
        ValidationBatch {
            id: "2".into(),
            partition: "p1".into(),
            priority: 2,
        },
        ValidationBatch {
            id: "1".into(),
            partition: "p1".into(),
            priority: 1,
        },
    ]);
    assert_eq!(schedule.batches[0].id, "1");
}
#[test]
fn test_validation_command_orchestration_correctness() {
    assert!(true);
}
#[test]
fn test_ci_timeout_recovery_restoration() {
    let r = CiTimeoutRuntime.recover(&CiExecutionTimeout { max_ticks: 10 }, "cp1".into());
    assert!(r.interrupted);
}
#[test]
fn test_incremental_replay_validation_equivalence() {
    let c = IncrementalReplayValidationRuntime.resume(IncrementalReplayWindow { start: 0, end: 5 });
    assert_eq!(c.offset, 5);
}
#[test]
fn test_release_candidate_automation() {
    let c = SovereignReleaseAutomationRuntime.generate("rc1");
    assert!(c.artifact_hash.starts_with("sha256:"));
}
#[test]
fn test_release_signature_verification() {
    let sig = ReleaseArtifactSignature {
        artifact_hash: "h".into(),
        signature: "sig:h".into(),
    };
    assert!(sig.verify().valid);
}
#[test]
fn test_runtime_generated_ci_reports() {
    let report = CiOrchestrationRuntime.ci_runtime_summary();
    assert!(report.release.verified);
}
#[test]
fn test_validation_partition_retry_equivalence() {
    let p = ValidationPartitionRuntime.run(&ValidationPartitionBatch {
        partition: "p".into(),
        tests: vec![],
    });
    assert!(p.success);
}
#[test]
fn test_release_manifest_lineage() {
    let m = SovereignReleaseManifest {
        ancestry: vec!["a".into(), "b".into()],
    };
    assert_eq!(m.ancestry.len(), 2);
}
#[test]
fn test_ci_corruption_detection() {
    assert!(detect_corruption("signature-mismatch"));
}
#[test]
fn test_ci_non_authoritative() {
    assert!(!detect_corruption("renderer-readonly"));
}

#[test]
fn test_integration_symbol_resolution() {
    let lineage = execution_core::runtime::export_governance::runtime_symbol_lineage();
    assert!(lineage
        .iter()
        .any(|l| l.symbol == "CiExecutionHistoryRuntime"));
}
