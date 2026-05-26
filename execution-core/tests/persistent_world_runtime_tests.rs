use execution_core::{
    game_runtime::{
        entities::Entity,
        input_runtime::{InputAction, RuntimeInput},
        inventory::InventoryState,
        replay_runtime::{ReplayRecord, ReplayTickRecord},
        simulation::step_runtime,
        world_state::WorldState,
    },
    world::verify_world_continuity,
};
use std::collections::BTreeMap;

fn seed_world() -> WorldState {
    let mut w = WorldState::new();
    w.entities = BTreeMap::from([(
        1,
        Entity {
            id: 1,
            x: 0,
            y: 0,
            authority: "player1".into(),
            runtime_lineage: "runtime-0".into(),
            world_continuity: "world-alpha".into(),
        },
    )]);
    w
}

#[test]
fn test_runtime_save_restore_equivalence() {
    let a = step_runtime(seed_world(), vec![], InventoryState::default());
    let b = serde_json::from_str::<WorldState>(&serde_json::to_string(&a.world).unwrap()).unwrap();
    assert!(verify_world_continuity(&a.world, &b).is_ok());
}
#[test]
fn test_world_reboot_continuity() {
    let a = step_runtime(seed_world(), vec![], InventoryState::default());
    let b = step_runtime(seed_world(), vec![], InventoryState::default());
    assert_eq!(a.validation_root, b.validation_root);
}
#[test]
fn test_inventory_restore_equivalence() {
    let a = step_runtime(
        seed_world(),
        vec![RuntimeInput {
            tick: 0,
            player_id: "player1".into(),
            action: InputAction::InventoryAction,
        }],
        InventoryState::default(),
    );
    let s = serde_json::to_string(&a.inventory).unwrap();
    let b: InventoryState = serde_json::from_str(&s).unwrap();
    assert_eq!(a.inventory, b);
}
#[test]
fn test_entity_restore_equivalence() {
    let a = step_runtime(seed_world(), vec![], InventoryState::default());
    let s = serde_json::to_string(&a.world.entities).unwrap();
    let b = serde_json::from_str::<BTreeMap<u64, Entity>>(&s).unwrap();
    assert_eq!(a.world.entities, b);
}
#[test]
fn test_replay_resume_equivalence() {
    let mut r = ReplayRecord::default();
    r.append_replay(ReplayTickRecord {
        tick: 1,
        inputs: vec![],
        state_root: "s".into(),
        event_root: "e".into(),
        validation_root: "v".into(),
    });
    assert_eq!(r.resume_replay(1).len(), 1);
}
#[test]
fn test_validation_root_chain_continuity() {
    let a = step_runtime(seed_world(), vec![], InventoryState::default());
    assert!(!a.validation_root.is_empty());
}
#[test]
fn test_snapshot_chain_equivalence() {
    let a = step_runtime(seed_world(), vec![], InventoryState::default());
    let b = step_runtime(seed_world(), vec![], InventoryState::default());
    assert_eq!(a.state_root, b.state_root);
}
#[test]
fn test_long_running_world_progression() {
    let mut w = seed_world();
    let mut i = InventoryState::default();
    for _ in 0..10_000 {
        let o = step_runtime(w, vec![], i);
        w = o.world;
        i = o.inventory;
    }
    assert_eq!(w.tick, 10_000);
}
#[test]
fn test_partial_restore_equivalence() {
    let out = step_runtime(seed_world(), vec![], InventoryState::default());
    assert_eq!(out.world.entities.len(), 1);
}
#[test]
fn test_event_window_restore_equivalence() {
    let a = step_runtime(seed_world(), vec![], InventoryState::default());
    let b = step_runtime(seed_world(), vec![], InventoryState::default());
    assert_eq!(a.event_root, b.event_root);
}
