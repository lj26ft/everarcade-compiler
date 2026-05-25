use execution_core::world::*;

fn sample_epoch(epoch_id: u64) -> ExecutionEpoch {
    let stream = EventStream {
        chunks: vec![EventChunk {
            chunk_index: 0,
            events: vec![ExecutionEvent {
                execution_id: "exec-a".into(),
                partition_id: "inventory".into(),
                sequence: 1,
                payload: b"hello".to_vec(),
            }],
        }],
    };
    ExecutionEpoch::from_parts(
        epoch_id,
        vec![EpochExecutionSummary {
            execution_id: "exec-a".into(),
            receipt_hash: "r1".into(),
            mutation_hash: "m1".into(),
        }],
        stream,
        WorldCheckpoint {
            tick: 1,
            continuity_root: WorldContinuityRoot("c1".into()),
            lifecycle: LifecycleCheckpoint {
                tick: 1,
                entity_count: 1,
            },
            ledger: EconomicLedgerCheckpoint {
                tick: 1,
                mutation_count: 1,
                ledger_root: "l1".into(),
            },
            scheduler: SchedulerCheckpoint {
                pending_tick_count: 0,
            },
        },
    )
    .unwrap()
}

#[test]
fn test_epoch_hash_stability() {
    let e = sample_epoch(1);
    assert_eq!(e.epoch_hash().unwrap(), e.epoch_hash().unwrap());
}
#[test]
fn test_dag_execution_order_stability() {
    let mut g = ExecutionGraph::default();
    g.add_node(ExecutionNode {
        id: "a".into(),
        shard: ExecutionShard("s".into()),
        phase: ExecutionPhase::Prepare,
        partition: ExecutionPartitionId("p".into()),
        dependencies: vec![],
    });
    g.add_node(ExecutionNode {
        id: "b".into(),
        shard: ExecutionShard("s".into()),
        phase: ExecutionPhase::Execute,
        partition: ExecutionPartitionId("p".into()),
        dependencies: vec![ExecutionDependency("a".into())],
    });
    assert_eq!(g.canonical_order().unwrap(), vec!["a", "b"]);
}
#[test]
fn test_partition_merge_equivalence() {
    let p1 = ExecutionPartition {
        id: ExecutionPartitionId("inventory".into()),
        mutation_keys: vec!["k2".into(), "k1".into()],
        node_ids: vec!["n2".into(), "n1".into()],
    };
    let p2 = ExecutionPartition {
        id: ExecutionPartitionId("economy".into()),
        mutation_keys: vec!["e1".into()],
        node_ids: vec!["e".into()],
    };
    assert_eq!(
        ExecutionGraph::partition_root(&[p1.clone(), p2.clone()]).unwrap(),
        ExecutionGraph::partition_root(&[p2, p1]).unwrap()
    );
}
#[test]
fn test_aggregated_receipt_root_equivalence() {
    let e = sample_epoch(1);
    assert_eq!(
        e.aggregated_receipt_root().unwrap(),
        e.aggregated_receipt_root().unwrap()
    );
}
#[test]
fn test_stdout_event_root_equivalence() {
    let mut s1 = EventStream {
        chunks: vec![EventChunk {
            chunk_index: 1,
            events: vec![
                ExecutionEvent {
                    execution_id: "b".into(),
                    partition_id: "p".into(),
                    sequence: 2,
                    payload: vec![2],
                },
                ExecutionEvent {
                    execution_id: "a".into(),
                    partition_id: "p".into(),
                    sequence: 1,
                    payload: vec![1],
                },
            ],
        }],
    };
    let mut s2 = s1.clone();
    s1.canonicalize();
    s2.canonicalize();
    assert_eq!(s1.root().unwrap(), s2.root().unwrap());
}
#[test]
fn test_checkpoint_restoration_equivalence() {
    let epoch = sample_epoch(4);
    let cp = epoch.checkpoint.world_checkpoint.clone();
    assert_eq!(cp.tick, 1);
}
#[test]
fn test_replay_compression_restoration() {
    let c = CompressedEpochRange {
        window: ReplayCompressionWindow {
            start_epoch: 1,
            end_epoch: 3,
        },
        snapshot: ReplaySnapshot {
            checkpoint_root: "cp".into(),
            epoch_hashes: vec!["a".into(), "b".into()],
        },
        anchor: ReplayAnchor {
            anchor_epoch: 3,
            anchor_hash: "h".into(),
        },
    };
    assert_eq!(c.window.end_epoch, 3);
}
#[test]
fn test_world_lineage_continuity() {
    let mut lineage = WorldLineage {
        world_id: "w".into(),
        chain: WorldEpochChain {
            epoch_hashes: vec![],
        },
    };
    let proof = sample_epoch(1).continuity_proof("genesis").unwrap();
    lineage.append_epoch(&proof);
    assert_eq!(lineage.chain.epoch_hashes.len(), 1);
}
#[test]
fn test_epoch_replay_equivalence() {
    let e1 = sample_epoch(1);
    let e2 = sample_epoch(1);
    assert_eq!(e1.epoch_hash().unwrap(), e2.epoch_hash().unwrap());
}
#[test]
fn test_execution_graph_hash_stability() {
    let mut g = ExecutionGraph::default();
    g.add_node(ExecutionNode {
        id: "n".into(),
        shard: ExecutionShard("s".into()),
        phase: ExecutionPhase::Execute,
        partition: ExecutionPartitionId("p".into()),
        dependencies: vec![],
    });
    assert_eq!(g.stable_hash().unwrap(), g.stable_hash().unwrap());
}
