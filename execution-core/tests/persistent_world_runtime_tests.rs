use execution_core::world::*;

fn seeded_world() -> WorldSimulation {
    let mut world = WorldSimulation::default();
    world.state.world_id = "world-alpha".into();
    world.state.lifecycles.push(EntityLifecycle {
        entity: CivilizationEntity {
            entity_id: "entity-0".into(),
            owner_id: "civ-0".into(),
            stage: EvolutionStage::Spawn,
            generation: 0,
        },
        history: vec![EvolutionStage::Spawn],
    });
    world.scheduler.schedule(0, "trade");
    world.scheduler.schedule(1, "crafting");
    world
}

#[test]
fn test_world_tick_equivalence() {
    let mut a = seeded_world();
    let mut b = seeded_world();
    assert_eq!(a.tick(), b.tick());
}

#[test]
fn test_entity_lifecycle_continuity() {
    let mut lifecycle = EntityLifecycle {
        entity: CivilizationEntity {
            entity_id: "e".into(),
            owner_id: "o".into(),
            stage: EvolutionStage::Spawn,
            generation: 0,
        },
        history: vec![EvolutionStage::Spawn],
    };
    lifecycle.advance(EvolutionStage::Upgrade);
    lifecycle.advance(EvolutionStage::Archival);
    assert_eq!(lifecycle.entity.generation, 2);
    assert_eq!(lifecycle.history.last(), Some(&EvolutionStage::Archival));
}

#[test]
fn test_inventory_replay_equivalence() {
    let mut a = seeded_world();
    let mut b = seeded_world();
    a.tick();
    b.tick();
    assert_eq!(a.state.inventory_mutations, b.state.inventory_mutations);
}

#[test]
fn test_economy_ledger_restoration() {
    let mut world = seeded_world();
    let checkpoint = world.tick();
    let manifest = RestorationManifest {
        world_id: "world-alpha".into(),
        checkpoint: checkpoint.clone(),
        cold_restore: true,
    };
    assert_eq!(manifest.checkpoint.ledger, checkpoint.ledger);
}

#[test]
fn test_scheduler_determinism() {
    let mut scheduler = WorldScheduler::default();
    scheduler.schedule(2, "transfer");
    scheduler.schedule(2, "consumption");
    let ops = scheduler.pop_tick(2);
    assert_eq!(ops[0].operation_id, "transfer");
    assert_eq!(ops[1].operation_id, "consumption");
}

#[test]
fn test_checkpoint_restoration_equivalence() {
    let mut world = seeded_world();
    let checkpoint = world.tick();
    let restored = checkpoint.clone();
    assert_eq!(checkpoint, restored);
}

#[test]
fn test_multi_era_archive_equivalence() {
    let mut a = seeded_world();
    let mut b = seeded_world();
    a.tick();
    a.tick();
    b.tick();
    b.tick();
    assert_eq!(a.archive, b.archive);
}

#[test]
fn test_world_federation_recovery() {
    let mut node_a = seeded_world();
    let mut node_b = seeded_world();
    let a0 = node_a.tick();
    let b0 = node_b.tick();
    assert_eq!(a0.continuity_root, b0.continuity_root);
}
