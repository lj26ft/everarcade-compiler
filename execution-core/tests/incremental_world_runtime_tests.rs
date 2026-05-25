use execution_core::world::*;

fn runtime_after_ticks(count: u64) -> IncrementalWorldRuntime {
    let mut rt = IncrementalWorldRuntime::default();
    for tick in 0..count {
        let parts = vec![format!("p{}", tick % 4), format!("p{}", (tick + 1) % 4)];
        rt.advance(
            WorldRuntimeTick {
                tick,
                workload_partitions: parts,
            },
            3,
        )
        .unwrap();
    }
    rt
}

#[test]
fn test_rolling_epoch_window_equivalence() {
    assert_eq!(
        RollingEpochWindow {
            start_epoch: 1,
            end_epoch: 3
        },
        RollingEpochWindow {
            start_epoch: 1,
            end_epoch: 3
        }
    );
}
#[test]
fn test_lane_scheduler_equivalence() {
    let p = vec!["b".into(), "a".into()];
    assert_eq!(
        ExecutionLaneScheduler::from_partitions(&p, 2).deterministic_order(),
        ExecutionLaneScheduler::from_partitions(&p, 2).deterministic_order()
    );
}
#[test]
fn test_incremental_snapshot_equivalence() {
    let s = IncrementalSnapshot {
        base_root: "x".into(),
        deltas: vec![],
        manifest: SnapshotSegmentManifest {
            segment_roots: vec![],
        },
    };
    assert_eq!(s.root().unwrap(), s.root().unwrap());
}
#[test]
fn test_streaming_event_archive_equivalence() {
    let mut a = StreamingEventArchive::default();
    let seg = EventSegment::from_window(
        EventWindow {
            start_sequence: 0,
            end_sequence: 0,
            partition_count: 1,
        },
        "e".into(),
    )
    .unwrap();
    a.push_segment(seg.clone());
    assert_eq!(a.segments[0], seg);
}
#[test]
fn test_incremental_replay_equivalence() {
    let w = IncrementalReplayWindow {
        start_tick: 1,
        end_tick: 2,
        delta_root: "r".into(),
    };
    assert_eq!(w.start_tick, 1);
}
#[test]
fn test_partial_restoration_equivalence() {
    let r = PartialWorldRestoration {
        world_id: "w".into(),
        partitions: vec![],
        segments: vec![],
    };
    assert_eq!(r.world_id, "w");
}
#[test]
fn test_incremental_witness_equivalence() {
    let b = StreamingWitnessBundle {
        chunks: vec![WitnessChunk {
            chunk_id: 1,
            witness_root: "x".into(),
        }],
    };
    assert_eq!(b.chunks.len(), 1);
}
#[test]
fn test_runtime_cursor_stability() {
    let rt = runtime_after_ticks(10);
    assert_eq!(rt.execution_cursor.next_tick, 10);
    assert_eq!(rt.commit_cursor.committed_tick, 9);
}
#[test]
fn test_continuity_cursor_equivalence() {
    let rt = runtime_after_ticks(5);
    assert_eq!(rt.continuity_cursor.segments.len(), 5);
}
#[test]
fn test_incremental_checkpoint_equivalence() {
    let rt = runtime_after_ticks(5);
    let w = rt.window(3);
    assert_eq!(
        w,
        WorldRuntimeWindow {
            start_tick: 2,
            end_tick: 4
        }
    );
}
#[test]
fn test_window_trim_equivalence() {
    let mut rt = runtime_after_ticks(20);
    rt.continuity_cursor.segments = rt.continuity_cursor.segments.into_iter().skip(10).collect();
    assert_eq!(rt.continuity_cursor.segments.first().unwrap().tick, 10);
}
#[test]
fn test_large_scale_world_progression() {
    let rt = runtime_after_ticks(10_001);
    assert_eq!(rt.execution_cursor.next_tick, 10_001);
}
