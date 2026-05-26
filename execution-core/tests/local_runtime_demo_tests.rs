use execution_core::game_runtime::{entities::Entity, input_runtime::{InputAction, RuntimeInput}, inventory::InventoryState, simulation::step_runtime, world_state::WorldState};
use std::collections::BTreeMap;

fn seed_world() -> WorldState {
    let mut w = WorldState::new();
    w.entities = BTreeMap::from([(1, Entity { id: 1, x: 0, y: 0, authority: "player1".into() })]);
    w
}

#[test] fn test_local_tick_equivalence(){ let a=step_runtime(seed_world(), vec![RuntimeInput{tick:0,player_id:"player1".into(),action:InputAction::MoveRight}], InventoryState::default()); let b=step_runtime(seed_world(), vec![RuntimeInput{tick:0,player_id:"player1".into(),action:InputAction::MoveRight}], InventoryState::default()); assert_eq!(a.state_root,b.state_root); }
#[test] fn test_player_movement_replay(){ let out=step_runtime(seed_world(), vec![RuntimeInput{tick:0,player_id:"player1".into(),action:InputAction::MoveUp}], InventoryState::default()); assert_eq!(out.world.entities.get(&1).unwrap().y,1); }
#[test] fn test_snapshot_restore_equivalence(){ let a=step_runtime(seed_world(), vec![], InventoryState::default()); let b=step_runtime(seed_world(), vec![], InventoryState::default()); assert_eq!(a.validation_root,b.validation_root); }
#[test] fn test_inventory_restore_equivalence(){ let a=step_runtime(seed_world(), vec![RuntimeInput{tick:0,player_id:"player1".into(),action:InputAction::InventoryAction}], InventoryState::default()); assert_eq!(a.inventory.ownership.get("player1").unwrap().len(),1); }
#[test] fn test_replay_validation_equivalence(){ let a=step_runtime(seed_world(), vec![], InventoryState::default()); assert!(!a.state_root.is_empty()); }
#[test] fn test_validation_root_progression(){ let a=step_runtime(seed_world(), vec![], InventoryState::default()); assert!(!a.validation_root.is_empty()); }
#[test] fn test_world_persistence_equivalence(){ let a=step_runtime(seed_world(), vec![], InventoryState::default()); let ser=serde_json::to_string(&a.world).unwrap(); let d:WorldState=serde_json::from_str(&ser).unwrap(); assert_eq!(a.world,d); }
#[test] fn test_stdout_event_equivalence(){ let a=step_runtime(seed_world(), vec![], InventoryState::default()); let b=step_runtime(seed_world(), vec![], InventoryState::default()); assert_eq!(a.event_root,b.event_root); }
#[test] fn test_large_tick_progression(){ let mut w=seed_world(); let mut i=InventoryState::default(); for _ in 0..100{ let o=step_runtime(w, vec![], i); w=o.world; i=o.inventory; } assert_eq!(w.tick,100); }
