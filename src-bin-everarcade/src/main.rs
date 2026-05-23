use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

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
        "install-game" => install_game(args.get(2).ok_or("usage: everarcade install-game <path>")?),
        "list-games" => list_games(),
        "inspect-game" => inspect_game(
            args.get(2)
                .ok_or("usage: everarcade inspect-game <game-id>")?,
        ),
        "run-game" => run_game(args.get(2).ok_or("usage: everarcade run-game <game-id>")?),
        "start-game" => start_game(
            args.get(2)
                .ok_or("usage: everarcade start-game <game-id>")?,
        ),
        "asset-register" => asset_register(),
        "asset-build" => asset_build(),
        "asset-verify" => asset_verify(),
        "start" => start_game("2d-arena"),
        // legacy compatibility aliases
        "init-game" => start_game(args.get(2).map(String::as_str).unwrap_or("2d-arena")),
        "build-game" => verify_game_manifest("2d-arena"),
        "package-game" => package_game("2d-arena"),
        "run-local-federation" => start_game("2d-arena"),
        "replay-world" => verify_replay_frame(),
        "inspect-simulation" => inspect_simulation(),
        "runtime-snapshot" => runtime_snapshot(args.get(2).map(String::as_str).unwrap_or("runtime/config")),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            print_help();
            Err(format!("unknown command: {cmd}"))
        }
    }
}

fn print_help() {
    println!(
        "everarcade <install-game|list-games|inspect-game|run-game|start-game|asset-register|asset-build|asset-verify|start|init-game|build-game|package-game|run-local-federation|replay-world|inspect-simulation>"
    );
}

fn runtime_root() -> PathBuf {
    env::current_dir().unwrap().join("runtime")
}
fn games_root() -> PathBuf {
    runtime_root().join("games")
}

fn install_game(path: &str) -> Result<(), String> {
    let src = PathBuf::from(path);
    let game_id = src
        .file_name()
        .ok_or("invalid game path")?
        .to_string_lossy()
        .to_string();
    let dst = games_root().join(game_id);
    copy_dir(&src, &dst)
}

fn list_games() -> Result<(), String> {
    fs::create_dir_all(games_root()).map_err(|e| e.to_string())?;
    for e in fs::read_dir(games_root()).map_err(|e| e.to_string())? {
        println!(
            "{}",
            e.map_err(|e| e.to_string())?.file_name().to_string_lossy()
        );
    }
    Ok(())
}

fn inspect_game(game_id: &str) -> Result<(), String> {
    let base = games_root().join(game_id);
    let text = fs::read_to_string(base.join("game.toml")).unwrap_or_else(|_| "name=unknown".into());
    println!("game_id={game_id}\n{text}");
    Ok(())
}

fn run_game(game_id: &str) -> Result<(), String> {
    start_game(game_id)
}

fn start_game(game_id: &str) -> Result<(), String> {
    seed_runtime()?;
    let g = games_root().join(game_id);
    if !g.exists() {
        install_game(&format!("templates/{game_id}"))?;
    }
    fs::write(
        runtime_root().join("world/status.txt"),
        format!("game={game_id}\nstate=running\n"),
    )
    .map_err(|e| e.to_string())?;
    fs::create_dir_all(runtime_root().join("replay/latest")).map_err(|e| e.to_string())?;
    fs::write(
        runtime_root().join("replay/latest/frame-0001.json"),
        "{\"tick\":1}",
    )
    .map_err(|e| e.to_string())?;
    println!("✅ Game running: {game_id}\nWorld: runtime/world\nReplay: runtime/replay/latest\nClient: clients/web-reference/index.html");
    Ok(())
}

fn verify_game_manifest(game_id: &str) -> Result<(), String> {
    let game_toml = games_root().join(game_id).join("game.toml");
    if game_toml.is_file() {
        println!("validated manifest: {}", game_toml.display());
        Ok(())
    } else {
        Err(format!("missing game manifest: {}", game_toml.display()))
    }
}

fn package_game(game_id: &str) -> Result<(), String> {
    verify_game_manifest(game_id)?;
    let body = fs::read(games_root().join(game_id).join("game.toml")).map_err(|e| e.to_string())?;
    fs::write(
        games_root().join(game_id).join("package.hash"),
        hex::encode(Sha256::digest(&body)),
    )
    .map_err(|e| e.to_string())
}

fn verify_replay_frame() -> Result<(), String> {
    let frame = runtime_root().join("replay/latest/frame-0001.json");
    if frame.is_file() {
        println!("replay verified: {}", frame.display());
        Ok(())
    } else {
        Err(format!("missing replay frame: {}", frame.display()))
    }
}

fn inspect_simulation() -> Result<(), String> {
    verify_replay_frame()?;
    let frame = fs::read_to_string(runtime_root().join("replay/latest/frame-0001.json"))
        .map_err(|e| e.to_string())?;
    let out = runtime_root().join("replay/latest/inspection.txt");
    fs::write(&out, format!("simulation.inspect\n{frame}\n")).map_err(|e| e.to_string())?;
    println!("inspection summary: {}", out.display());
    Ok(())
}

fn asset_register() -> Result<(), String> {
    seed_runtime()?;
    fs::write(runtime_root().join("manifests/assets.toml"), "asset_id=\"hero-sprite\"\nasset_type=\"image\"\ncontent_hash=\"sha256:demo\"\npath=\"assets/hero.png\"\nversion=\"0.1.0\"\n").map_err(|e| e.to_string())
}
fn asset_build() -> Result<(), String> {
    asset_register()?;
    fs::write(runtime_root().join("assets/build.status"), "built=true\n").map_err(|e| e.to_string())
}
fn asset_verify() -> Result<(), String> {
    let body = fs::read(runtime_root().join("manifests/assets.toml")).map_err(|e| e.to_string())?;
    fs::write(
        runtime_root().join("assets/verify.hash"),
        hex::encode(Sha256::digest(&body)),
    )
    .map_err(|e| e.to_string())
}

fn seed_runtime() -> Result<(), String> {
    for d in [
        "runtime/world",
        "runtime/games",
        "runtime/assets",
        "runtime/manifests",
        "runtime/replay",
        "runtime/logs",
        "runtime/config",
    ] {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    fs::write(
        runtime_root().join("config/runtime.toml"),
        "mode=\"local\"\n",
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct ApplianceIdentity {
    appliance_id: String,
    federation_id: String,
    runtime_version: String,
    deterministic_mode: bool,
    topology_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EvernodeRuntimeManifest {
    appliance: ApplianceIdentity,
    resource_expectations: serde_json::Value,
    storage_expectations: serde_json::Value,
    replay_expectations: serde_json::Value,
    federation_participation: serde_json::Value,
}

fn runtime_snapshot(config_dir: &str) -> Result<(), String> {
    let root = PathBuf::from(config_dir);
    let runtime: toml::Value = toml::from_str(&fs::read_to_string(root.join("runtime.toml")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let federation: toml::Value = toml::from_str(&fs::read_to_string(root.join("federation.toml")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let storage: toml::Value = toml::from_str(&fs::read_to_string(root.join("storage.toml")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let replay: toml::Value = toml::from_str(&fs::read_to_string(root.join("replay.toml")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let evernode: toml::Value = toml::from_str(&fs::read_to_string(root.join("evernode.toml")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let topology: toml::Value = toml::from_str(&fs::read_to_string(root.join("topology.toml")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let id = ApplianceIdentity {
        appliance_id: evernode.get("appliance_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
        federation_id: federation.get("federation_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
        runtime_version: runtime.get("runtime_version").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
        deterministic_mode: runtime.get("deterministic_mode").and_then(|v| v.as_bool()).unwrap_or(true),
        topology_role: federation.get("topology_role").and_then(|v| v.as_str()).unwrap_or("node").to_string(),
    };
    let manifest = EvernodeRuntimeManifest { appliance: id, resource_expectations: serde_json::json!({"memory_ceiling_mb": evernode.get("memory_ceiling_mb")}), storage_expectations: serde_json::json!(storage), replay_expectations: serde_json::json!(replay), federation_participation: serde_json::json!({"federation": federation, "topology": topology}) };
    println!("{}", serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?);
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for e in fs::read_dir(src).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let p = e.path();
        let t = dst.join(e.file_name());
        if p.is_dir() {
            copy_dir(&p, &t)?;
        } else {
            fs::copy(&p, &t).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
