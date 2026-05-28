use execution_core::{
    civilization::{
        recovery::restore_civilization, validation::reject_replay_authority_mutation,
        CivilizationRuntimeState,
    },
    economy_runtime::{
        recovery::restore_economy, validation::reject_settlement_mutation, EconomyRuntime,
    },
    entity_runtime::{EntityRuntime, SovereignEntity},
    entity_scheduler::{
        priority::EntityPriority, recovery::restore_schedule,
        validation::reject_scheduling_mutation, EntitySchedulerRuntime,
    },
    inventory_runtime::{
        recovery::restore_inventory, validation::reject_ownership_mutation, InventoryRuntime,
    },
    persistent_multiplayer::{
        validation::reject_replay_authority_mutation as reject_multiplayer_authority_mutation,
        world_sync::WorldSyncState, PersistentMultiplayerRuntime,
    },
    world_persistence::{
        checkpoint::WorldCheckpoint, restoration::restore_checkpoint,
        validation::reject_non_append_only, WorldPersistenceRuntime,
    },
    world_recovery::SovereignWorldRecovery,
    world_runtime::{validation::validate_world_equivalence, PersistentWorldRuntime},
};

#[test]
fn test_world_tick_equivalence() {
    let mut a = PersistentWorldRuntime::new("atlas");
    let mut b = PersistentWorldRuntime::new("atlas");
    a.tick("input:a").unwrap();
    b.tick("input:a").unwrap();
    validate_world_equivalence(&a.world, &b.world).unwrap();
}

#[test]
fn test_entity_identity_continuity() {
    let e = SovereignEntity::genesis("entity-a");
    assert_eq!(e.identity_root, "entity:entity-a:identity");
    assert_eq!(
        e.lineage_root,
        "entity:entity-a:lineage:0:entity:entity-a:identity"
    );
}

#[test]
fn test_entity_evolution_equivalence() {
    let mut a = EntityRuntime::new(&["e1", "e2"]);
    let mut b = EntityRuntime::new(&["e1", "e2"]);
    a.evolve_all("tick-root").unwrap();
    b.evolve_all("tick-root").unwrap();
    assert_eq!(a.entities, b.entities);
}

#[test]
fn test_civilization_replay_restoration() {
    let mut state = CivilizationRuntimeState::genesis("civ");
    state.tick("harvest").unwrap();
    let restored = restore_civilization(&state, &state).unwrap();
    assert_eq!(state, restored);
}

#[test]
fn test_economy_ledger_equivalence() {
    let mut a = EconomyRuntime::genesis();
    let mut b = EconomyRuntime::genesis();
    a.transfer("treasury", "market", 7).unwrap();
    b.transfer("treasury", "market", 7).unwrap();
    restore_economy(&a, &b).unwrap();
}

#[test]
fn test_inventory_ownership_continuity() {
    let mut a = InventoryRuntime::genesis("item-1", "owner-a");
    let mut b = InventoryRuntime::genesis("item-1", "owner-a");
    a.transfer("owner-b").unwrap();
    b.transfer("owner-b").unwrap();
    restore_inventory(&a, &b).unwrap();
}

#[test]
fn test_world_checkpoint_restoration() {
    let mut persistence = WorldPersistenceRuntime::new();
    persistence.archive_replay("replay:1");
    let checkpoint = WorldCheckpoint::new(1, "world-root", "replay:1");
    persistence.persist_checkpoint(checkpoint.clone()).unwrap();
    assert_eq!(restore_checkpoint(&checkpoint).unwrap(), checkpoint);
}

#[test]
fn test_entity_scheduler_equivalence() {
    let inputs = vec![
        EntityPriority {
            priority: 1,
            entity_id: "b".into(),
        },
        EntityPriority {
            priority: 1,
            entity_id: "a".into(),
        },
        EntityPriority {
            priority: 0,
            entity_id: "z".into(),
        },
    ];
    let mut a = EntitySchedulerRuntime::new();
    let mut b = EntitySchedulerRuntime::new();
    assert_eq!(a.schedule(inputs.clone()).unwrap(), vec!["z", "a", "b"]);
    b.schedule(inputs).unwrap();
    restore_schedule(&a, &b).unwrap();
}

#[test]
fn test_persistent_multiplayer_continuity() {
    let mut runtime = PersistentMultiplayerRuntime::new(vec![
        WorldSyncState::new("peer-a", "world:0", "replay:0"),
        WorldSyncState::new("peer-b", "world:0", "replay:0"),
    ]);
    runtime.sync("world:1", "replay:1").unwrap();
    assert!(runtime.continuity_root.contains("world:1"));
}

#[test]
fn test_world_divergence_rejection() {
    let mut a = PersistentWorldRuntime::new("atlas");
    let mut b = PersistentWorldRuntime::new("atlas");
    a.tick("input:a").unwrap();
    b.tick("input:b").unwrap();
    assert!(validate_world_equivalence(&a.world, &b.world).is_err());
    assert!(reject_non_append_only(2, 1).is_err());
}

#[test]
fn test_authority_mutation_rejection() {
    let runtime = PersistentWorldRuntime::new("atlas");
    assert!(runtime.unauthorized_mutation().is_err());
    assert!(EntityRuntime::new(&["e"]).unauthorized_mutation().is_err());
    assert!(reject_replay_authority_mutation(true).is_err());
    assert!(reject_settlement_mutation(false).is_err());
    assert!(reject_ownership_mutation(false).is_err());
    assert!(reject_scheduling_mutation(false).is_err());
    assert!(reject_multiplayer_authority_mutation(true).is_err());
}

#[test]
fn test_observer_world_hydration() {
    let recovery = SovereignWorldRecovery::new(
        "civilization-root",
        "entity-lineage-root",
        "economy-root",
        "inventory-root",
        "replay-tip",
    );
    assert_eq!(recovery.restore().unwrap(), recovery);
    let hydrated = format!("observer-world:hydrated:{}", recovery.replay_tip);
    assert_eq!(hydrated, "observer-world:hydrated:replay-tip");
}
