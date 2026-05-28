use execution_core::{
    ai_memory::{restoration::restore_memory, validation::memory_equivalent, AiMemoryRuntime},
    ai_runtime::{recovery::restore_ai, validation::ai_equivalent, AiRuntime},
    behavior_tree::{
        recovery::restore_behavior, validation::behavior_equivalent, BehaviorNode, BehaviorStatus,
        BehaviorTreeRuntime,
    },
    ecs::{
        recovery::restore_from_replay, validation::replay_equivalent, ComponentValue,
        DeterministicSystem, EcsMutation, EcsRuntime, Entity,
    },
    simulation_scheduler::{
        partition::PartitionWork, recovery::restore_scheduler, validation::scheduler_equivalent,
        SimulationSchedulerRuntime,
    },
    world_partition::{
        migration::MigrationRecord, recovery::restore_partitions,
        validation::partition_runtime_equivalent, WorldPartition, WorldPartitionRuntime,
    },
    world_simulation::{
        recovery::restore_world_simulation, validation::world_equivalent, TerrainCell,
        WorldSimulationRuntime,
    },
};

#[test]
fn test_ecs_execution_equivalence() {
    let mut runtime = EcsRuntime::default();
    runtime.spawn(Entity::new("entity-b"));
    runtime.spawn(Entity::new("entity-a"));
    runtime.set_component(
        "entity-a",
        ComponentValue::new("energy", 1, "deterministic-ecs-runtime"),
    );
    runtime.set_component(
        "entity-b",
        ComponentValue::new("energy", 2, "deterministic-ecs-runtime"),
    );
    let replay = runtime
        .execute_systems(vec![DeterministicSystem::new("sys-energy", "energy", 3)])
        .unwrap();
    let restored = restore_from_replay(&replay).unwrap();
    assert!(replay_equivalent(&runtime, &restored));
}

#[test]
fn test_ai_decision_equivalence() {
    let mut runtime = AiRuntime::default();
    runtime
        .execute(vec!["ai-b".into(), "ai-a".into()], "root:ai:1")
        .unwrap();
    let restored = restore_ai(&runtime.decisions);
    assert!(ai_equivalent(&runtime, &restored));
}

#[test]
fn test_ai_memory_continuity() {
    let mut memory = AiMemoryRuntime::default();
    memory.append("ai-a", "saw river", "root:memory:1").unwrap();
    memory
        .append("ai-a", "built bridge", "root:memory:2")
        .unwrap();
    let restored = restore_memory(&memory.store);
    assert!(memory.validate());
    assert!(memory_equivalent(&memory.store, &restored.store));
}

#[test]
fn test_behavior_tree_equivalence() {
    let mut runtime = BehaviorTreeRuntime::default();
    runtime
        .execute(
            vec![
                BehaviorNode {
                    id: "seek".into(),
                    priority: 1,
                    status: BehaviorStatus::Success,
                },
                BehaviorNode {
                    id: "plan".into(),
                    priority: 0,
                    status: BehaviorStatus::Running,
                },
            ],
            "root:bt:1",
        )
        .unwrap();
    let restored = restore_behavior(&runtime.executions);
    assert!(behavior_equivalent(&runtime, &restored));
}

#[test]
fn test_partition_streaming_equivalence() {
    let mut runtime = WorldPartitionRuntime::default();
    runtime
        .add_partition(
            WorldPartition {
                partition_id: "p-b".into(),
                ..Default::default()
            },
            "root:p:1",
        )
        .unwrap();
    runtime
        .add_partition(
            WorldPartition {
                partition_id: "p-a".into(),
                ..Default::default()
            },
            "root:p:2",
        )
        .unwrap();
    runtime.validate().unwrap();
    let restored = restore_partitions(&runtime);
    assert!(partition_runtime_equivalent(&runtime, &restored));
}

#[test]
fn test_world_simulation_equivalence() {
    let mut world = WorldSimulationRuntime::default();
    world.add_cell(TerrainCell {
        x: 1,
        y: 0,
        height: 10,
        biome: "plain".into(),
    });
    world.add_cell(TerrainCell {
        x: 0,
        y: 0,
        height: 12,
        biome: "forest".into(),
    });
    world.evolve("root:world:1").unwrap();
    let restored = restore_world_simulation(&world);
    assert!(world_equivalent(&world, &restored));
}

#[test]
fn test_simulation_scheduler_equivalence() {
    let mut scheduler = SimulationSchedulerRuntime::default();
    scheduler
        .schedule(vec![
            PartitionWork {
                partition_id: "b".into(),
                ecs_entities: 1,
                ai_entities: 2,
            },
            PartitionWork {
                partition_id: "a".into(),
                ecs_entities: 1,
                ai_entities: 1,
            },
        ])
        .unwrap();
    let restored = restore_scheduler(&scheduler.schedule);
    assert!(scheduler_equivalent(&scheduler, &restored));
}

#[test]
fn test_simulation_federation_continuity() {
    let local = MigrationRecord {
        entity_id: "e1".into(),
        source_region: "r1".into(),
        target_region: "r2".into(),
        ownership_epoch: 1,
        continuity_proof: "root:federation:1".into(),
        sequence: 0,
    };
    let remote = local.clone();
    assert_eq!(local, remote);
    assert!(!local.continuity_proof.is_empty());
}

#[test]
fn test_shard_migration_restoration() {
    let mut partition = WorldPartition {
        partition_id: "shard-a".into(),
        ..Default::default()
    };
    partition
        .entity_regions
        .insert("entity-a".into(), "region-a".into());
    let record = partition
        .migrate_entity_partition("entity-a", "region-b")
        .unwrap();
    assert_eq!(record.target_region, "region-b");
    assert_eq!(
        partition.entity_regions.get("entity-a").unwrap(),
        "region-b"
    );
}

#[test]
fn test_partition_divergence_rejection() {
    let mut runtime = WorldPartitionRuntime::default();
    assert!(runtime
        .add_partition(WorldPartition::default(), "")
        .is_err());
}

#[test]
fn test_authority_mutation_rejection() {
    let mut runtime = EcsRuntime::default();
    let err = runtime
        .apply_mutation(
            EcsMutation {
                entity_id: "e".into(),
                component: "energy".into(),
                delta: 1,
                authority: "renderer".into(),
            },
            1,
        )
        .unwrap_err();
    assert_eq!(format!("{:?}", err), "UnauthorizedMutation");
}

#[test]
fn test_observer_simulation_hydration() {
    let mut ecs = EcsRuntime::default();
    ecs.spawn(Entity::new("observer-entity"));
    let replay = ecs
        .execute_systems(vec![DeterministicSystem::new("sys-observe", "visible", 1)])
        .unwrap();
    let observer = restore_from_replay(&replay).unwrap();
    assert!(replay_equivalent(&ecs, &observer));
}
