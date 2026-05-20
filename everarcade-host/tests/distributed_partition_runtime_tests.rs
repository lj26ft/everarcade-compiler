use execution_core::world_partition::{orchestration, replay, scheduler, WorldPartition};

#[test]
fn distributed_partition_runtime_flow() {
    let mut a = WorldPartition {
        partition_id: "A".into(),
        ..Default::default()
    };
    let mut b = WorldPartition {
        partition_id: "B".into(),
        ..Default::default()
    };
    let mut c = WorldPartition {
        partition_id: "C".into(),
        ..Default::default()
    };

    for p in [&mut a, &mut b, &mut c] {
        p.assign_region_owner("region-a", "node-1", "root");
        p.entity_regions
            .insert(format!("entity-{}", p.partition_id), "region-a".into());
        p.resolve_cross_partition_interaction("boot");
    }

    let _ = a.migrate_entity_partition("entity-A", "region-a").err();
    let schedule = scheduler::schedule_partition_execution("A", 1, vec!["world-event".into()]);
    assert!(scheduler::verify_partition_schedule(&schedule));

    let state = orchestration::orchestrate_entity_execution(&a);
    assert!(orchestration::verify_orchestration_continuity(&state));

    assert!(replay::verify_partition_convergence(&[a, b, c]));
}
