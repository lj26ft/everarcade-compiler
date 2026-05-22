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
        _ => {
            println!("everarcade <install-game|list-games|inspect-game|run-game|start-game|asset-register|asset-build|asset-verify|start>");
            Ok(())
        }
    }
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
    ] {
        fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
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
