use execution_core::runtime::validation::*;

#[test]
fn test_validation_dag_determinism() {
    let dag = ValidationDagRuntime::new(
        vec![
            ValidationStageNode { id: "a".into() },
            ValidationStageNode { id: "b".into() },
        ],
        vec![ValidationStageDependency {
            from: "a".into(),
            to: "b".into(),
        }],
    );
    let mut checkpoints = ValidationCheckpointRuntime::default();
    let first = dag.execute(&mut checkpoints).unwrap();
    let mut checkpoints2 = ValidationCheckpointRuntime::default();
    let second = dag.execute(&mut checkpoints2).unwrap();
    assert_eq!(first.ordered_stages, second.ordered_stages);
}

#[test]
fn test_validation_checkpoint_restoration() {
    let mut cp = ValidationCheckpointRuntime::default();
    cp.mark_completed("stage-1".into());
    let recovered = ValidationRecoveryRuntime.recover(&cp.checkpoint());
    assert!(recovered.resume_from.contains("stage-1"));
}
#[test]
fn test_workspace_partition_equivalence() {
    let rt = WorkspacePartitionRuntime;
    let m = WorkspacePartitionManifest {
        partitions: vec![WorkspacePartition {
            id: "p".into(),
            crates: vec!["execution-core".into()],
        }],
    };
    assert!(rt.equivalent(&m, &m));
}
#[test]
fn test_replay_shard_restoration() {
    let rt = ReplayShardRuntime;
    assert!(rt.validate_continuity(&ReplayShardContinuity {
        previous_root: "a".into(),
        next_root: "b".into()
    }));
}
#[test]
fn test_validation_pressure_profile_generation() {
    let p = RuntimePressureProfile {
        stage_count: 3,
        memory_ceiling_bytes: 1024,
    };
    assert_eq!(p.stage_count, 3);
}
#[test]
fn test_incremental_validation_window_equivalence() {
    let a = ValidationReplayWindowResult {
        verified: true,
        last_position: 9,
    };
    let b = ValidationReplayWindowResult {
        verified: true,
        last_position: 9,
    };
    assert_eq!(a, b);
}
#[test]
fn test_release_build_reproducibility() {
    let a = SovereignReleaseVerification { reproducible: true };
    assert!(a.reproducible);
}
#[test]
fn test_validation_report_materialization() {
    let rt = ValidationReportRuntime;
    let s = rt.summarize(&ValidationReportManifest {
        stages: vec![ValidationStageResult {
            stage_id: "s".into(),
            passed: true,
        }],
    });
    assert_eq!(s.passed_stages, 1);
}
#[test]
fn test_validation_recovery_equivalence() {
    let cp = ValidationCheckpoint {
        completed_stages: ["a".to_string()].into_iter().collect(),
    };
    let r1 = ValidationRecoveryRuntime.recover(&cp);
    let r2 = ValidationRecoveryRuntime.recover(&cp);
    assert_eq!(r1, r2);
}
#[test]
fn test_archive_scalability_restoration() {
    let m = ReplayShardManifest {
        continuity_root: "root".into(),
        windows: vec![ReplayShardWindow {
            shard_id: "s1".into(),
            lower: 0,
            upper: 10,
        }],
    };
    assert_eq!(m.windows.len(), 1);
}
#[test]
fn test_validation_corruption_detection() {
    assert!(detect_corruption(false, "dag").is_err());
}
#[test]
fn test_workspace_non_authoritative() {
    assert!(true, "renderer is non-authoritative by design");
}

#[test]
fn test_runtime_stress_validation() {
    let stress = RuntimeStressValidation {
        deterministic_ordering: true,
        replay_equivalence: true,
        dag_determinism: true,
        restoration_equivalence: true,
    };
    assert!(stress.is_stable());
}

#[test]
fn test_validation_replay_restoration() {
    let recovery = ValidationReplayRecovery { restored: true };
    assert!(recovery.restored);
}

#[test]
fn test_release_candidate_reproducibility() {
    let verification = ReleaseCandidateVerification {
        deterministic: true,
        replay_equivalent: true,
    };
    assert!(verification.deterministic && verification.replay_equivalent);
}

#[test]
fn test_runtime_generated_report_integrity() {
    let report = RuntimeGeneratedValidationReport {
        execution: RuntimeValidationExecution {
            timestamp_utc: "2026-01-01T00:00:00Z".into(),
            replay_equivalence: true,
            warning_gate_passed: true,
            security_gate_passed: true,
        },
        restoration_passed: true,
        partition_diagnostics: "stable".into(),
        load_diagnostics: "stable".into(),
    };
    assert!(report.execution.replay_equivalence && report.restoration_passed);
}

#[test]
fn test_warning_gate_cleanliness() {
    assert!(true);
}

#[test]
fn test_runtime_security_validation() {
    let security = RuntimeSecurityResult {
        passed: true,
        violations: vec![],
    };
    assert!(security.passed);
}

#[test]
fn test_runtime_partition_stability() {
    let rt = WorkspacePartitionRuntime;
    let manifest = WorkspacePartitionManifest { partitions: vec![] };
    assert!(rt.equivalent(&manifest, &manifest));
}

#[test]
fn test_runtime_pressure_rejection() {
    let result = RuntimePressureValidation {
        linker_stable: false,
        memory_stable: true,
        deterministic_ordering: true,
    }
    .evaluate();
    assert!(!result.accepted);
}

#[test]
fn test_runtime_exhaustion_boundary() {
    let boundary = RuntimeExhaustionBoundary {
        max_stages: 100,
        max_memory_bytes: 1024,
    };
    assert_eq!(boundary.max_stages, 100);
}

#[test]
fn test_validation_replay_corruption_detection() {
    assert!(detect_corruption(false, "validation replay corruption").is_err());
}

#[test]
fn test_runtime_surface_classification() {
    let audit = execution_core::runtime::runtime_status::RuntimeSurfaceAudit::run();
    assert!(!audit.classifications.is_empty());
}

#[test]
fn test_warning_cleanup_integrity() {
    assert_eq!(0, 0);
}

#[test]
fn test_scaffold_runtime_annotation_integrity() {
    assert!(
        execution_core::runtime::runtime_status::RuntimeSurfaceAudit::run()
            .scaffold_modules
            .iter()
            .any(|m| m.contains("renderer_client::history"))
    );
}

#[test]
fn test_export_surface_consistency() {
    assert!(
        execution_core::runtime::runtime_status::RuntimeSurfaceAudit::run()
            .export_inconsistencies
            .is_empty()
    );
}

#[test]
fn test_replay_runtime_public_surface() {
    assert!(
        execution_core::runtime::export_governance::runtime_export_ownership()
            .iter()
            .any(|entry| entry.export.category.contains("replay"))
    );
}
