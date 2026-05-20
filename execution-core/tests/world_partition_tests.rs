use execution_core::world_partition::{
    orchestration, replay, scheduler, verification, WorldPartition,
};

fn sample_partition(id: &str) -> WorldPartition {
    let mut p = WorldPartition {
        partition_id: id.to_string(),
        ..Default::default()
    };
    p.assign_region_owner("region-a", "node-1", "root-a");
    p.assign_region_owner("region-b", "node-2", "root-b");
    p.entity_regions
        .insert("player-001".into(), "region-a".into());
    p
}

#[test]
fn test_entity_partition_migration() {
    let mut p = sample_partition("A");
    let rec = p
        .migrate_entity_partition("player-001", "region-b")
        .unwrap();
    verification::verify_partition_migration(&rec).unwrap();
}

#[test]
fn test_region_ownership_transfer() {
    let mut p = sample_partition("A");
    p.transfer_region_ownership("region-a", "node-9").unwrap();
    let owner = p.ownership.get("region-a").unwrap();
    verification::verify_region_continuity(owner, "node-9").unwrap();
}

#[test]
fn test_partition_replay_convergence() {
    let mut a = sample_partition("A");
    let mut b = sample_partition("B");
    a.resolve_cross_partition_interaction("event-1");
    b.resolve_cross_partition_interaction("event-1");
    assert!(replay::verify_partition_convergence(&[a, b]));
}

#[test]
fn test_cross_partition_event_replay() {
    let mut p = sample_partition("A");
    p.propagate_partition_event(1);
    p.propagate_partition_event(2);
    assert!(verification::verify_partition_interaction(
        &p.event_sequences
    ));
}

#[test]
fn test_partition_load_balancing() {
    let mut a = sample_partition("C");
    let b = sample_partition("A");
    a.entity_regions.insert("npc-1".into(), "region-a".into());
    let mut all = vec![a, b];
    orchestration::balance_partition_load(&mut all);
    assert_eq!(all[0].partition_id, "A");
}

#[test]
fn test_autonomous_entity_orchestration() {
    let p = sample_partition("A");
    let state = orchestration::orchestrate_entity_execution(&p);
    assert!(orchestration::verify_orchestration_continuity(&state));
}

#[test]
fn test_large_scale_simulation_convergence() {
    let schedule = scheduler::schedule_partition_execution(
        "A",
        1,
        vec!["econ".into(), "combat".into(), "migrate".into()],
    );
    assert!(scheduler::verify_partition_schedule(&schedule));
}

#[test]
fn test_long_running_partition_replay() {
    let mut p = sample_partition("A");
    for i in 0..100 {
        p.propagate_partition_event(i);
        p.resume_partition_execution();
    }
    assert_eq!(p.tick, 100);
    assert!(replay::reconstruct_world_partitions(&[p]));
}
