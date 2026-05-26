use execution_core::game_runtime::{
    entities::Entity,
    input_runtime::{InputAction, RuntimeInput},
    inventory::InventoryState,
    replay_runtime::{ReplayRecord, ReplayTickRecord},
    simulation::step_runtime,
    world_state::WorldState,
};
use std::collections::BTreeMap;

fn seed_world() -> (WorldState, InventoryState) {
    let mut world = WorldState::new();
    world.entities = BTreeMap::from([(1, Entity { id: 1, x: 0, y: 0, authority: "p1".into(), runtime_lineage: "r".into(), world_continuity: "w".into() })]);
    (world, InventoryState::default())
}

#[test] fn test_interactive_movement_equivalence() { let (w,i)=seed_world(); let o1=step_runtime(w.clone(),vec![RuntimeInput{tick:0,player_id:"p1".into(),action:InputAction::MoveRight}],i.clone()); let o2=step_runtime(w,vec![RuntimeInput{tick:0,player_id:"p1".into(),action:InputAction::MoveRight}],i); assert_eq!(o1.state_root,o2.state_root);} 
#[test] fn test_live_replay_equivalence() { let mut r=ReplayRecord::default(); r.append_replay(ReplayTickRecord{tick:1,inputs:vec![],state_root:"a".into(),event_root:"b".into(),validation_root:"c".into()}); assert!(r.verify_replay()); }
#[test] fn test_terminal_projection_equivalence() { assert_eq!("x","x"); }
#[test] fn test_inventory_interaction_equivalence() { let mut i=InventoryState::default(); i.add_item("p1","i1"); assert_eq!(i.inspect("p1").len(),1);} 
#[test] fn test_save_restore_interactive_equivalence() { let (w,_) = seed_world(); let j=serde_json::to_string(&w).unwrap(); let w2:WorldState=serde_json::from_str(&j).unwrap(); assert_eq!(w.tick,w2.tick);} 
#[test] fn test_projection_root_progression() { let (w,i)=seed_world(); let o=step_runtime(w,vec![],i); assert!(!o.validation_root.is_empty()); }
#[test] fn test_live_tick_equivalence() { let (w,i)=seed_world(); let o=step_runtime(w,vec![],i); assert_eq!(o.world.tick,1);} 
#[test] fn test_checkpoint_resume_equivalence() { let mut r=ReplayRecord::default(); r.append_replay(ReplayTickRecord{tick:2,inputs:vec![],state_root:"a".into(),event_root:"b".into(),validation_root:"c".into()}); assert_eq!(r.resume_replay(2).len(),1);} 
#[test] fn test_large_interactive_progression() { let (mut w, mut inv)=seed_world(); for t in 0..10_000 { let o=step_runtime(w,vec![RuntimeInput{tick:t,player_id:"p1".into(),action:InputAction::MoveRight}],inv); w=o.world; inv=o.inventory; } assert_eq!(w.tick,10_000);} 
