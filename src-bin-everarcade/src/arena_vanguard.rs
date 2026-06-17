use crate::config::runtime_root;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::PathBuf};

pub const WORLD_ID: &str = "arena-vanguard";
const TICK_RATE: u32 = 20;
const SPEED: i32 = 100;
const ATTACK_RANGE: i32 = 150;
const ATTACK_DAMAGE: i32 = 25;
const MAX_HEALTH: i32 = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    id: String,
    x: i32,
    y: i32,
    health: i32,
    connected: bool,
    deaths: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CombatEvent {
    tick: u64,
    attacker: String,
    target: String,
    damage: i32,
    target_health: i32,
    death: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorldState {
    world_id: String,
    tick_rate: u32,
    tick: u64,
    players: BTreeMap<String, Player>,
    combat_events: Vec<CombatEvent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum InputKind {
    Join,
    Disconnect,
    Move { direction: Direction },
    Attack { target: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Input {
    tick: u64,
    player: String,
    #[serde(flatten)]
    kind: InputKind,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Receipt {
    tick: u64,
    player: String,
    input_hash: String,
    pre_state_root: String,
    post_state_root: String,
    events: Vec<String>,
    accepted: bool,
}

pub fn run_local() -> Result<(), String> {
    let root = arena_root();
    fs::create_dir_all(runtime_root().join("world")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root.join("package")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root.join("projection")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root.join("proofs")).map_err(|e| e.to_string())?;
    write_static_package(&root)?;
    let inputs = demo_inputs();
    let (state, receipts, roots) = execute(&inputs)?;
    write_json(root.join("journal.json"), &inputs)?;
    write_json(root.join("state.json"), &state)?;
    write_json(root.join("receipts.json"), &receipts)?;
    write_json(root.join("proofs/proof-bundle.json"), &roots)?;
    fs::write(
        runtime_root().join("world/status.txt"),
        "game=arena-vanguard\nstate=running\nplayers=2\n",
    )
    .map_err(|e| e.to_string())?;
    println!(
        "arena_vanguard=running world={} projection={} proof={}",
        root.display(),
        root.join("projection/index.html").display(),
        root.join("proofs/proof-bundle.json").display()
    );
    Ok(())
}

pub fn replay() -> Result<(), String> {
    let root = arena_root();
    let inputs: Vec<Input> = serde_json::from_str(
        &fs::read_to_string(root.join("journal.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let (state, receipts, roots) = execute(&inputs)?;
    let saved: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("proofs/proof-bundle.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let fresh = serde_json::to_value(&roots).map_err(|e| e.to_string())?;
    if saved != fresh {
        return Err("arena replay divergence: proof roots differ".into());
    }
    write_json(root.join("replay-state.json"), &state)?;
    write_json(root.join("replay-receipts.json"), &receipts)?;
    println!(
        "arena_replay=verified state_root={} receipt_root={} world_hash={} continuity_root={}",
        roots.state_root, roots.receipt_root, roots.world_hash, roots.continuity_root
    );
    Ok(())
}

#[derive(Serialize)]
struct Roots {
    state_root: String,
    receipt_root: String,
    world_hash: String,
    continuity_root: String,
    package_hash: String,
}

fn execute(inputs: &[Input]) -> Result<(WorldState, Vec<Receipt>, Roots), String> {
    let mut state = WorldState {
        world_id: WORLD_ID.into(),
        tick_rate: TICK_RATE,
        tick: 0,
        players: BTreeMap::new(),
        combat_events: vec![],
    };
    let mut receipts = vec![];
    let mut ordered = inputs.to_vec();
    ordered.sort_by(|a, b| a.tick.cmp(&b.tick).then(a.player.cmp(&b.player)));
    for input in ordered {
        state.tick = input.tick;
        let pre = hash_json(&state)?;
        let mut events = vec![];
        match &input.kind {
            InputKind::Join => {
                state
                    .players
                    .entry(input.player.clone())
                    .and_modify(|p| p.connected = true)
                    .or_insert(Player {
                        id: input.player.clone(),
                        x: if input.player == "p1" { 0 } else { 100 },
                        y: 0,
                        health: MAX_HEALTH,
                        connected: true,
                        deaths: 0,
                    });
                events.push("player_joined".into());
            }
            InputKind::Disconnect => {
                if let Some(p) = state.players.get_mut(&input.player) {
                    p.connected = false;
                    events.push("player_disconnected".into());
                }
            }
            InputKind::Move { direction } => {
                if let Some(p) = state.players.get_mut(&input.player) {
                    if p.connected && p.health > 0 {
                        match direction {
                            Direction::Up => p.y -= SPEED,
                            Direction::Down => p.y += SPEED,
                            Direction::Left => p.x -= SPEED,
                            Direction::Right => p.x += SPEED,
                        };
                        events.push("player_moved".into());
                    }
                }
            }
            InputKind::Attack { target } => {
                let (ax, ay) = state
                    .players
                    .get(&input.player)
                    .map(|p| (p.x, p.y))
                    .unwrap_or((i32::MIN / 2, i32::MIN / 2));
                if let Some(t) = state.players.get_mut(target) {
                    let dx = ax - t.x;
                    let dy = ay - t.y;
                    if t.health > 0 && dx.abs() + dy.abs() <= ATTACK_RANGE {
                        t.health = (t.health - ATTACK_DAMAGE).max(0);
                        let death = t.health == 0;
                        if death {
                            t.deaths += 1;
                            events.push("death".into());
                        }
                        events.push("damage_applied".into());
                        state.combat_events.push(CombatEvent {
                            tick: input.tick,
                            attacker: input.player.clone(),
                            target: target.clone(),
                            damage: ATTACK_DAMAGE,
                            target_health: t.health,
                            death,
                        });
                    }
                }
            }
        }
        let post = hash_json(&state)?;
        receipts.push(Receipt {
            tick: input.tick,
            player: input.player.clone(),
            input_hash: hash_json(&input)?,
            pre_state_root: pre,
            post_state_root: post,
            events,
            accepted: true,
        });
    }
    let state_root = hash_json(&state)?;
    let receipt_root = hash_json(&receipts)?;
    let package_hash = hash_bytes(include_bytes!("../../templates/arena-vanguard/world.toml"));
    let world_hash = hash_bytes(format!("{state_root}{package_hash}").as_bytes());
    let continuity_root = hash_bytes(format!("{state_root}{receipt_root}{world_hash}").as_bytes());
    Ok((
        state,
        receipts,
        Roots {
            state_root,
            receipt_root,
            world_hash,
            continuity_root,
            package_hash,
        },
    ))
}

fn demo_inputs() -> Vec<Input> {
    vec![
        Input {
            tick: 1,
            player: "p1".into(),
            kind: InputKind::Join,
        },
        Input {
            tick: 1,
            player: "p2".into(),
            kind: InputKind::Join,
        },
        Input {
            tick: 2,
            player: "p1".into(),
            kind: InputKind::Move {
                direction: Direction::Right,
            },
        },
        Input {
            tick: 3,
            player: "p1".into(),
            kind: InputKind::Attack {
                target: "p2".into(),
            },
        },
        Input {
            tick: 4,
            player: "p2".into(),
            kind: InputKind::Disconnect,
        },
        Input {
            tick: 5,
            player: "p2".into(),
            kind: InputKind::Join,
        },
    ]
}
fn arena_root() -> PathBuf {
    runtime_root().join("games/arena-vanguard")
}
fn write_json<T: Serialize>(path: PathBuf, value: &T) -> Result<(), String> {
    fs::write(
        path,
        serde_json::to_string_pretty(value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
fn hash_json<T: Serialize>(value: &T) -> Result<String, String> {
    Ok(hash_bytes(
        serde_json::to_string(value)
            .map_err(|e| e.to_string())?
            .as_bytes(),
    ))
}
fn hash_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn write_static_package(root: &std::path::Path) -> Result<(), String> {
    for file in ["world.toml", "genesis.json", "contract-package.json"] {
        fs::copy(
            format!("templates/arena-vanguard/{file}"),
            root.join("package").join(file),
        )
        .map_err(|e| e.to_string())?;
    }
    fs::write(
        root.join("projection/index.html"),
        include_str!("../../templates/arena-vanguard/projection/index.html"),
    )
    .map_err(|e| e.to_string())
}
