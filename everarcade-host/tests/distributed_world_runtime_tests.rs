use execution_core::world_scheduler::{
    event::{WorldEvent, WorldEventKind},
    migration::{export_entity_state, import_entity_state, resume_entity_execution},
    ownership::{
        assign_entity_owner, transfer_entity_ownership, verify_entity_continuity,
        EntityOwnershipRegistry,
    },
    scheduler::{replay_world_timeline, verify_world_convergence, WorldScheduler},
};

#[test]
fn test_distributed_tick_convergence() {
    let mut a = WorldScheduler::default();
    let mut b = WorldScheduler::default();
    let mut c = WorldScheduler::default();
    for s in 1..=3 {
        let ev = WorldEvent {
            sequence: s,
            kind: WorldEventKind::World,
            subject_id: "world".into(),
            payload: vec![s as u8],
        };
        a.schedule_world_event(ev.clone());
        b.schedule_world_event(ev.clone());
        c.schedule_world_event(ev);
    }
    for _ in 0..3 {
        let _ = a.execute_world_tick().unwrap();
        let _ = b.execute_world_tick().unwrap();
        let _ = c.execute_world_tick().unwrap();
    }
    assert!(verify_world_convergence(&a.timeline, &b.timeline));
    assert!(verify_world_convergence(&b.timeline, &c.timeline));
}

#[test]
fn test_world_event_replay() {
    let mut scheduler = WorldScheduler::default();
    scheduler.schedule_world_event(WorldEvent {
        sequence: 1,
        kind: WorldEventKind::Entity,
        subject_id: "player-001".into(),
        payload: b"spawn".to_vec(),
    });
    let _ = scheduler.execute_world_tick().unwrap();
    let replayed = replay_world_timeline(&scheduler.timeline);
    assert!(verify_world_convergence(&scheduler.timeline, &replayed));
}

#[test]
fn test_entity_ownership_transfer() {
    let mut reg = EntityOwnershipRegistry::default();
    assign_entity_owner(&mut reg, "player-001".into(), "node-a".into(), [1u8; 32]);
    let before = reg.entities.get("player-001").unwrap().clone();
    let after =
        transfer_entity_ownership(&mut reg, "player-001", "node-b".into(), [2u8; 32]).unwrap();
    assert!(verify_entity_continuity(&before, &after));
}

#[test]
fn test_entity_migration_between_nodes() {
    let blob = export_entity_state("player-001", b"continuity-state");
    let imported = import_entity_state(blob);
    assert!(resume_entity_execution(&imported));
}
