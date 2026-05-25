use execution_core::world::{
    assert_lane_equivalence, assert_replay_equivalence, assert_restoration_equivalence,
    assert_runtime_equivalence, assert_snapshot_equivalence, assert_witness_equivalence,
    runtime_validation_root, ExecutionMetrics, RuntimeMetrics, RuntimeValidationReport,
};

fn sample_metrics(executions: u64, windows: u64) -> RuntimeMetrics {
    RuntimeMetrics {
        execution: ExecutionMetrics {
            execution_count: executions,
            partition_count: 8,
            aggregated_receipt_root: "receipt-root".into(),
            aggregated_mutation_root: "mutation-root".into(),
        },
        epoch: execution_core::world::EpochMetrics {
            epoch_count: 128,
            checkpoint_lineage_root: "lineage-root".into(),
        },
        lane: execution_core::world::LaneMetrics {
            lane_count: 16,
            deterministic_merge_equivalence: true,
        },
        replay: execution_core::world::ReplayMetrics {
            replay_window_count: windows,
            replay_equivalence: true,
        },
        restoration: execution_core::world::RestorationMetrics {
            restoration_equivalence: true,
            partial_restoration_window_count: windows,
        },
        witness: execution_core::world::WitnessMetrics {
            witness_chunk_count: 5_000,
            aggregated_witness_root: "witness-root".into(),
        },
        event: execution_core::world::EventMetrics {
            event_chunk_count: 5_000,
            aggregated_event_root: "event-root".into(),
        },
        snapshot: execution_core::world::SnapshotMetrics {
            snapshot_count: windows,
            snapshot_chain_root: "snapshot-root".into(),
        },
        continuity: execution_core::world::ContinuityMetrics {
            continuity_equivalence: true,
            continuity_root: "continuity-root".into(),
        },
    }
}

#[test]
fn test_large_scale_runtime_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert_eq!(m.execution.execution_count, 50_000);
}
#[test]
fn test_multi_window_equivalence() {
    let m = sample_metrics(50_000, 120);
    assert_eq!(m.replay.replay_window_count, 120);
}
#[test]
fn test_lane_merge_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert!(assert_lane_equivalence(&m, &m).is_ok());
}
#[test]
fn test_snapshot_chain_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert!(assert_snapshot_equivalence(&m, &m).is_ok());
}
#[test]
fn test_replay_chain_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert!(assert_replay_equivalence(&m, &m).is_ok());
}
#[test]
fn test_streaming_event_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert_eq!(m.event.event_chunk_count, 5_000);
}
#[test]
fn test_witness_chunk_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert!(assert_witness_equivalence(&m, &m).is_ok());
}
#[test]
fn test_incremental_restoration_equivalence() {
    let m = sample_metrics(50_000, 100);
    assert!(assert_restoration_equivalence(&m, &m).is_ok());
}
#[test]
fn test_runtime_validation_root_stability() {
    let m = sample_metrics(50_000, 100);
    let r1 = runtime_validation_root(&m).unwrap();
    let r2 = runtime_validation_root(&m).unwrap();
    let report1 = RuntimeValidationReport::from_metrics(m.clone()).unwrap();
    let report2 = RuntimeValidationReport::from_metrics(m).unwrap();
    assert_eq!(r1.0, r2.0);
    assert!(assert_runtime_equivalence(&report1, &report2).is_ok());
}
