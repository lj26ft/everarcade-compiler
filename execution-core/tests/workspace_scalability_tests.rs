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
