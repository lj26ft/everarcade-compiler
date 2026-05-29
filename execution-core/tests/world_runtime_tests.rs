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

#[test]
fn test_persistent_world_equivalence() {
    use execution_core::live_world_ops::{LiveUpdatePlan, LiveWorldPlatform, WorldLifecycleState};

    let mut platform = LiveWorldPlatform::create("atlas-live");
    platform.boot().unwrap();
    assert_eq!(platform.lifecycle, WorldLifecycleState::Booted);
    platform.tick("input:harvest").unwrap();
    platform.shutdown().unwrap();
    platform.restart().unwrap();

    let mut mirror = LiveWorldPlatform::create("atlas-live");
    mirror.boot().unwrap();
    mirror.tick("input:harvest").unwrap();
    let plan = LiveUpdatePlan::new(1, 2, &platform.world_runtime.world);
    assert!(plan.validate());
    assert_eq!(platform.world_runtime.world, mirror.world_runtime.world);
}

#[test]
fn test_multiplayer_session_equivalence() {
    use execution_core::live_world_ops::LiveWorldPlatform;

    let mut a = LiveWorldPlatform::create("coop");
    let mut b = LiveWorldPlatform::create("coop");
    assert_eq!(a.host().unwrap(), b.host().unwrap());
    assert_eq!(a.join("player-a").unwrap(), b.join("player-a").unwrap());
    assert_eq!(a.join("player-b").unwrap(), b.join("player-b").unwrap());
    assert_eq!(a.multiplayer, b.multiplayer);
}

#[test]
fn test_session_continuity() {
    use execution_core::live_world_ops::{validate_session, LiveWorldPlatform};

    let mut platform = LiveWorldPlatform::create("session-world");
    platform.host().unwrap();
    let session = platform.join("alice").unwrap();
    assert!(validate_session(&session));
    assert_eq!(session.restore().unwrap(), session);
}

#[test]
fn test_world_recovery_equivalence() {
    use execution_core::live_world_ops::{LiveWorldPlatform, WorldLifecycleState};

    let mut platform = LiveWorldPlatform::create("recoverable");
    platform.host().unwrap();
    platform.tick("input:one").unwrap();
    let before = platform.world_runtime.world.clone();
    platform.persist_current_checkpoint().unwrap();
    platform.recover_from_latest().unwrap();
    assert_eq!(platform.lifecycle, WorldLifecycleState::Hosted);
    assert_eq!(platform.world_runtime.world, before);
}

#[test]
fn test_world_upgrade_equivalence() {
    use execution_core::live_world_ops::{LiveUpdatePlan, LiveWorldPlatform};

    let mut platform = LiveWorldPlatform::create("upgradeable");
    platform.host().unwrap();
    let plan = LiveUpdatePlan::new(7, 8, &platform.world_runtime.world);
    platform.migrate(&plan).unwrap();
    assert!(platform
        .world_runtime
        .world
        .replay_tip
        .contains(&plan.migration_root));
    assert!(plan.validate());
}

#[test]
fn test_runtime_monitoring_equivalence() {
    use execution_core::live_world_ops::LiveWorldPlatform;

    let mut platform = LiveWorldPlatform::create("monitored");
    platform.host().unwrap();
    platform.join("operator").unwrap();
    platform.deploy("monitored-game", 1).unwrap();
    let dashboard = platform.dashboard();
    assert_eq!(dashboard.online_players, 1);
    assert!(dashboard.validate(&platform.world_runtime.world));
    assert_eq!(dashboard.deployment_health, "live");
}

#[test]
fn test_deployment_automation_equivalence() {
    use execution_core::live_world_ops::LiveWorldPlatform;

    let mut a = LiveWorldPlatform::create("deployable");
    let mut b = LiveWorldPlatform::create("deployable");
    a.host().unwrap();
    b.host().unwrap();
    let deployed_a = a.deploy("published-game", 3).unwrap();
    let deployed_b = b.deploy("published-game", 3).unwrap();
    assert_eq!(deployed_a, deployed_b);
    assert_eq!(deployed_a.status, "live");
    assert!(deployed_a.verified_root.ends_with(":live"));
}

#[test]
fn test_live_world_operations() {
    use execution_core::live_world_ops::{AdminAction, LiveWorldPlatform};

    let mut platform = LiveWorldPlatform::create("operated");
    platform.host().unwrap();
    platform.join("moderator").unwrap();
    let receipt = platform
        .apply_admin_action(AdminAction::UpdateWorldSetting {
            key: "motd".into(),
            value: "hello".into(),
        })
        .unwrap();
    assert!(receipt.action_root.contains("admin:setting"));
    assert_eq!(platform.admin_log.len(), 1);
    assert!(platform.dashboard().validate(&platform.world_runtime.world));
}

#[test]
fn test_replay_safe_multiplayer() {
    use execution_core::{
        live_world_ops::LiveWorldPlatform,
        persistent_multiplayer::validation::reject_replay_authority_mutation,
        world_persistence::validation::reject_non_append_only,
    };

    let mut platform = LiveWorldPlatform::create("replay-safe");
    platform.host().unwrap();
    platform.join("peer-a").unwrap();
    let old_len = platform.persistence.archive.entries.len();
    platform.tick("input:deterministic").unwrap();
    assert!(reject_non_append_only(old_len, platform.persistence.archive.entries.len()).is_ok());
    assert!(reject_replay_authority_mutation(true).is_err());
}

#[test]
fn test_marketplace_runtime_integration() {
    use execution_core::live_world_ops::LiveWorldPlatform;

    let mut platform = LiveWorldPlatform::create("market-world");
    platform.host().unwrap();
    platform.deploy("market-game", 1).unwrap();
    let listing = platform
        .marketplace_listing("creator:ada", "persistent-world")
        .unwrap();
    assert_eq!(listing.published_game, "market-game");
    assert_eq!(listing.live_world, "market-world");
    assert_eq!(listing.creator_profile, "creator:ada");
    assert_eq!(listing.template, "persistent-world");
}
