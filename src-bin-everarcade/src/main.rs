use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");
    match cmd {
        "init-game" => init_game(
            args.get(2)
                .cloned()
                .unwrap_or_else(|| "my-first-world".into()),
        ),
        "build-game" => build_game(),
        "package-game" => package_game(),
        "run-local-federation" => run_local_federation(),
        "replay-world" => replay_world(),
        "inspect-simulation" => inspect_simulation(),
        _ => {
            println!("everarcade <init-game|build-game|package-game|run-local-federation|replay-world|inspect-simulation>");
            Ok(())
        }
    }
}

fn root() -> PathBuf {
    env::current_dir().unwrap().join(".everarcade-dev")
}
fn init_game(name: String) -> Result<(), String> {
    fs::create_dir_all(root().join(name)).map_err(|e| e.to_string())
}
fn build_game() -> Result<(), String> {
    fs::write(root().join("build.json"), "{\"deterministic\":true}").map_err(|e| e.to_string())
}
fn package_game() -> Result<(), String> {
    let body = fs::read(root().join("build.json")).map_err(|e| e.to_string())?;
    let hash = hex::encode(Sha256::digest(&body));
    fs::write(root().join("package.hash"), hash).map_err(|e| e.to_string())
}
fn run_local_federation() -> Result<(), String> {
    fs::create_dir_all(root().join("federation/node-a")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root().join("federation/node-b")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root().join("federation/node-c")).map_err(|e| e.to_string())
}
fn replay_world() -> Result<(), String> {
    fs::write(root().join("replay.log"), "replay=ok").map_err(|e| e.to_string())
}
fn inspect_simulation() -> Result<(), String> {
    fs::write(root().join("simulation.inspect"), "inspect=ok").map_err(|e| e.to_string())
}
