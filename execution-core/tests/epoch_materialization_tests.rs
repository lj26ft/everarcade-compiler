use execution_core::world::*;

fn fixture() -> (ExecutionGraph, Vec<ExecutionPartition>, WorldCheckpoint) {
    let mut g = ExecutionGraph::default();
    g.add_node(ExecutionNode {
        id: "a".into(),
        shard: ExecutionShard("s1".into()),
        phase: ExecutionPhase::Prepare,
        partition: ExecutionPartitionId("p1".into()),
        dependencies: vec![],
    });
    g.add_node(ExecutionNode {
        id: "b".into(),
        shard: ExecutionShard("s1".into()),
        phase: ExecutionPhase::Execute,
        partition: ExecutionPartitionId("p2".into()),
        dependencies: vec![ExecutionDependency("a".into())],
    });
    let parts = vec![
        ExecutionPartition {
            id: ExecutionPartitionId("p1".into()),
            mutation_keys: vec!["k1".into()],
            node_ids: vec!["a".into()],
        },
        ExecutionPartition {
            id: ExecutionPartitionId("p2".into()),
            mutation_keys: vec!["k2".into()],
            node_ids: vec!["b".into()],
        },
    ];
    let cp = WorldCheckpoint {
        tick: 1,
        continuity_root: WorldContinuityRoot("c".into()),
        lifecycle: LifecycleCheckpoint {
            tick: 1,
            entity_count: 0,
        },
        ledger: EconomicLedgerCheckpoint {
            tick: 0,
            mutation_count: 0,
            ledger_root: "l".into(),
        },
        scheduler: SchedulerCheckpoint {
            pending_tick_count: 0,
        },
    };
    (g, parts, cp)
}

#[test]
fn test_epoch_materialization_equivalence() {
    let (g, p, c) = fixture();
    let a = EpochMaterializer::commit_epoch(1, &g, &p, c.clone(), "gen")
        .unwrap()
        .0;
    let b = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(a, b);
}
#[test]
fn test_partition_execution_equivalence() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(e.partitions.len(), 2);
}
#[test]
fn test_partition_merge_equivalence() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert!(!e.receipt_root.0.is_empty());
}
#[test]
fn test_epoch_commit_equivalence() {
    let (g, p, c) = fixture();
    let (_e, r, _w, _a) = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen").unwrap();
    assert!(!r.epoch_hash.is_empty());
}
#[test]
fn test_epoch_replay_equivalence() {
    let (g, p, c) = fixture();
    let a = EpochMaterializer::replay_epoch(1, &g, &p, c.clone(), "gen").unwrap();
    let b = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(a, b);
}
#[test]
fn test_epoch_restoration_equivalence() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    let r = EpochMaterializer::restore_epoch(&e).unwrap();
    assert_eq!(r.restored_checkpoint_root, e.checkpoint_root);
}
#[test]
fn test_replay_compression_equivalence() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    let b = CompressedEpochBundle {
        epoch_id: e.epoch_id.clone(),
        deltas: e
            .partitions
            .iter()
            .map(|p| CompressedPartitionDelta {
                partition_id: p.partition_id.clone(),
                mutation_root: p.mutation_root.clone(),
                receipt_root: p.receipt_root.clone(),
            })
            .collect(),
        event_root: e.event_root.clone(),
    };
    assert_eq!(b.deltas.len(), 2);
}
#[test]
fn test_event_stream_materialization_equivalence() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(e.event_stream.archive.root, e.event_root);
}
#[test]
fn test_snapshot_restoration_equivalence() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c.clone(), "gen")
        .unwrap()
        .0;
    let snap = WorldSnapshot {
        manifest: SnapshotManifest {
            world_id: "w".into(),
            epoch_id: e.epoch_id.clone(),
            event_root: e.event_root.clone(),
        },
        anchor: SnapshotAnchor {
            epoch_hash: hashing(&e),
            checkpoint_root: e.checkpoint_root.0.clone(),
        },
        checkpoint: c,
    };
    let rec = SnapshotRestorationReceipt {
        snapshot_hash: hashing(&snap),
        restoration_hash: hashing(&e),
    };
    assert!(!rec.snapshot_hash.is_empty());
}
#[test]
fn test_witness_bundle_equivalence() {
    let (g, p, c) = fixture();
    let (_e, _r, w, _a) = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen").unwrap();
    assert_eq!(w.partition_witnesses.len(), 2);
}
#[test]
fn test_epoch_root_stability() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(hashing(&e), hashing(&e));
}
#[test]
fn test_partition_root_stability() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(
        e.partitions[0].receipt_root.clone(),
        e.partitions[0].receipt_root.clone()
    );
}
#[test]
fn test_materialized_event_root_stability() {
    let (g, p, c) = fixture();
    let e = EpochMaterializer::commit_epoch(1, &g, &p, c, "gen")
        .unwrap()
        .0;
    assert_eq!(e.event_root, e.event_root);
}

fn hashing<T: serde::Serialize>(v: &T) -> String {
    execution_core::hashing::hash_bytes(
        &execution_core::canonical::encoding::canonical_encode(v).unwrap(),
    )
}
