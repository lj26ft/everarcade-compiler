mod console;
mod projection_service;
mod projection_status;
mod status;

use console::{parse_command, ConsoleCommand};
use execution_core::game_runtime::{
    entities::Entity,
    input_runtime::{InputAction, RuntimeInput},
    inventory::InventoryState,
    simulation::step_runtime,
    world_state::WorldState,
};
use std::{collections::BTreeMap, fs, path::PathBuf};

fn main() {
    let world_root = PathBuf::from("runtime/world");
    let _ = fs::create_dir_all(world_root.join("snapshots"));
    let mut world = WorldState::new();
    world.entities = BTreeMap::from([(
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
    let mut inventory = InventoryState::default();

    println!("runtime_boot: deterministic sovereign runtime started");
    let bootstrap_command = parse_command("status");
    if let Some(ConsoleCommand::Status) = bootstrap_command {
        println!("console_bootstrap: status command parser active");
    }

    let demo_inputs = vec![
        RuntimeInput {
            tick: 0,
            player_id: "player1".into(),
            action: InputAction::MoveRight,
        },
        RuntimeInput {
            tick: 0,
            player_id: "player1".into(),
            action: InputAction::MoveUp,
        },
        RuntimeInput {
            tick: 0,
            player_id: "player1".into(),
            action: InputAction::InventoryAction,
        },
    ];
    let out = step_runtime(world, demo_inputs, inventory);
    world = out.world;
    inventory = out.inventory;
    println!("tick_progression: {}", world.tick);
    println!(
        "validation_roots: state={} event={} validation={}",
        out.state_root, out.event_root, out.validation_root
    );
    println!(
        "inventory_changes: {}",
        serde_json::to_string(&inventory.ownership).unwrap()
    );
    println!(
        "status: {}",
        status::render_status(
            world.tick,
            world.entities.len(),
            &out.state_root,
            &out.event_root,
            &out.validation_root,
            &serde_json::to_string(&inventory.ownership).unwrap(),
            0,
            0
        )
    );
}
