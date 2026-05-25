use execution_core::world::*;

#[test]
fn deterministic_world_load_simulation() {
    let mut g = ExecutionGraph::default();
    let mut p1 = vec![];
    let mut p2 = vec![];
    for i in 0..1000u64 {
        let id = format!("n{i}");
        let pid = if i % 2 == 0 { "p1" } else { "p2" };
        let deps = if i == 0 {
            vec![]
        } else {
            vec![ExecutionDependency(format!("n{}", i - 1))]
        };
        g.add_node(ExecutionNode {
            id: id.clone(),
            shard: ExecutionShard("s".into()),
            phase: ExecutionPhase::Execute,
            partition: ExecutionPartitionId(pid.into()),
            dependencies: deps,
        });
        if i % 2 == 0 {
            p1.push(id)
        } else {
            p2.push(id)
        }
    }
    let parts = vec![
        ExecutionPartition {
            id: ExecutionPartitionId("p1".into()),
            mutation_keys: vec!["a".into()],
            node_ids: p1,
        },
        ExecutionPartition {
            id: ExecutionPartitionId("p2".into()),
            mutation_keys: vec!["b".into()],
            node_ids: p2,
        },
    ];
    let cp = WorldCheckpoint {
        tick: 9,
        continuity_root: WorldContinuityRoot("c".into()),
        lifecycle: LifecycleCheckpoint {
            tick: 9,
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
    let e1 = EpochMaterializer::commit_epoch(77, &g, &parts, cp.clone(), "prev")
        .unwrap()
        .0;
    let e2 = EpochMaterializer::commit_epoch(77, &g, &parts, cp, "prev")
        .unwrap()
        .0;
    assert_eq!(e1.receipt_root, e2.receipt_root);
    assert_eq!(e1.event_root, e2.event_root);
}
