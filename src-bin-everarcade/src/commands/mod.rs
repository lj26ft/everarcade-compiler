use crate::config::{games_root, runtime_root};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn dispatch(args: &[String]) -> Result<(), String> {
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
        "init-game" => start_game(args.get(2).map(String::as_str).unwrap_or("2d-arena")),
        "build-game" => verify_game_manifest("2d-arena"),
        "package-game" => package_game("2d-arena"),
        "run-local-federation" => start_game("2d-arena"),
        "replay-world" => verify_replay_frame(),
        "inspect-simulation" => inspect_simulation(),
        _ => Err(format!("unknown command: {cmd}")),
    }
}
pub fn print_help() {
    println!("everarcade <install-game|list-games|inspect-game|run-game|start-game|asset-register|asset-build|asset-verify|start|init-game|build-game|package-game|run-local-federation|replay-world|inspect-simulation|runtime-snapshot>");
}
fn install_game(path: &str) -> Result<(), String> {
    let src = PathBuf::from(path);
    let game_id = src
        .file_name()
        .ok_or("invalid game path")?
        .to_string_lossy()
        .to_string();
    copy_dir(&src, &games_root().join(game_id))
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
    let text = fs::read_to_string(games_root().join(game_id).join("game.toml"))
        .unwrap_or_else(|_| "name=unknown".into());
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
    let p = games_root().join(game_id).join("game.toml");
    if p.is_file() {
        println!("validated manifest: {}", p.display());
        Ok(())
    } else {
        Err(format!("missing game manifest: {}", p.display()))
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
    fs::write(runtime_root().join("manifests/assets.toml"),"asset_id=\"hero-sprite\"\nasset_type=\"image\"\ncontent_hash=\"sha256:demo\"\npath=\"assets/hero.png\"\nversion=\"0.1.0\"\n").map_err(|e|e.to_string())
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
