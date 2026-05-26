#![allow(dead_code)]
mod console;
mod event_view;
mod input_loop;
mod inventory_view;
mod playback;
mod projection_service;
mod projection_status;
mod render_tick;
mod render_validation;
mod renderer;
mod status;
mod world_view;

use execution_core::game_runtime::{
    entities::Entity,
    input_runtime::RuntimeInput,
    inventory::InventoryState,
    replay_runtime::{ReplayRecord, ReplayTickRecord},
    simulation::step_runtime,
    world_state::WorldState,
};
use input_loop::{InteractiveInputLoop, PlayerCommand};
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
    let mut replay = ReplayRecord::default();
    let mut input = InteractiveInputLoop::new("session-local");

    for line in ["move d", "move w", "inventory", "status"] {
        if let Some(frame) = input.parse_line(world.tick, "player1", line) {
            match frame.command {
                PlayerCommand::Runtime(ri) => {
                    let out = step_runtime(world.clone(), vec![ri.clone()], inventory.clone());
                    replay.append_replay(ReplayTickRecord {
                        tick: out.world.tick,
                        inputs: vec![RuntimeInput {
                            tick: ri.tick,
                            player_id: ri.player_id,
                            action: ri.action,
                        }],
                        state_root: out.state_root.clone(),
                        event_root: out.event_root.clone(),
                        validation_root: out.validation_root.clone(),
                    });
                    world = out.world;
                    inventory = out.inventory;
                    println!(
                        "tick={} root={} validation={}",
                        world.tick, out.state_root, out.validation_root
                    );
                }
                PlayerCommand::Status => println!(
                    "status: {}",
                    status::render_status(
                        world.tick,
                        world.entities.len(),
                        "n/a",
                        "n/a",
                        "n/a",
                        inventory.ownership.len(),
                        0,
                        replay.ticks.len(),
                        "checkpoint-0"
                    )
                ),
                _ => {}
            }
        }
    }

    println!(
        "interactive_runtime_ready session={} replay_ticks={}",
        input.session.session_id,
        replay.ticks.len()
    );
}
