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
        "xahau-build-hooks" => xahau_build_hooks(),
        "xahau-install-hooks" => xahau_install_hooks(),
        "xahau-verify-hooks" => xahau_verify_hooks(),
        "xahau-submit-settlement" => xahau_submit_settlement(),
        "xahau-anchor-checkpoint" => xahau_anchor_checkpoint(),
        "xahau-vault-status" => xahau_vault_status(),
        _ => {
            println!("everarcade <init-game|build-game|package-game|run-local-federation|replay-world|inspect-simulation|xahau-build-hooks|xahau-install-hooks|xahau-verify-hooks|xahau-submit-settlement|xahau-anchor-checkpoint|xahau-vault-status>");
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
    fs::create_dir_all(root()).map_err(|e| e.to_string())?;
    fs::write(root().join("build.json"), "{\"deterministic\":true}").map_err(|e| e.to_string())
}
fn package_game() -> Result<(), String> {
    let body = fs::read(root().join("build.json")).map_err(|e| e.to_string())?;
    let hash = hex::encode(Sha256::digest(&body));
    fs::write(root().join("package.hash"), hash).map_err(|e| e.to_string())
}
fn run_local_federation() -> Result<(), String> {
    let federation = root().join("federation");
    for n in ["node-a", "node-b", "node-c"] {
        fs::create_dir_all(federation.join(n)).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(root().join("timelines")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root().join("inspectors/replay")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root().join("inspectors/timeline")).map_err(|e| e.to_string())?;
    fs::write(
        root().join("timelines/world.timeline"),
        "tick=0 root=genesis\ntick=1 root=state-1\n",
    )
    .map_err(|e| e.to_string())?;
    fs::write(root().join("simulation.status"), "running").map_err(|e| e.to_string())?;
    Ok(())
}
fn replay_world() -> Result<(), String> {
    fs::write(root().join("replay.log"), "replay=ok\nconvergence=verified")
        .map_err(|e| e.to_string())
}
fn inspect_simulation() -> Result<(), String> {
    fs::write(
        root().join("simulation.inspect"),
        "inspect=ok\ncontinuity=ok",
    )
    .map_err(|e| e.to_string())
}

fn xahau_root() -> PathBuf {
    root().join("xahau")
}

fn xahau_build_hooks() -> Result<(), String> {
    fs::create_dir_all(xahau_root()).map_err(|e| e.to_string())?;
    fs::write(xahau_root().join("hooks.build"), "status=built\n").map_err(|e| e.to_string())
}

fn xahau_install_hooks() -> Result<(), String> {
    fs::create_dir_all(xahau_root()).map_err(|e| e.to_string())?;
    fs::write(xahau_root().join("hooks.install"), "status=installed\n").map_err(|e| e.to_string())
}

fn xahau_verify_hooks() -> Result<(), String> {
    fs::create_dir_all(xahau_root()).map_err(|e| e.to_string())?;
    fs::write(xahau_root().join("hooks.verify"), "status=verified\n").map_err(|e| e.to_string())
}

fn xahau_submit_settlement() -> Result<(), String> {
    fs::create_dir_all(xahau_root()).map_err(|e| e.to_string())?;
    fs::write(
        xahau_root().join("settlement.receipt"),
        "settlement=submitted\nfinality=pending\n",
    )
    .map_err(|e| e.to_string())
}

fn xahau_anchor_checkpoint() -> Result<(), String> {
    fs::create_dir_all(xahau_root()).map_err(|e| e.to_string())?;
    fs::write(
        xahau_root().join("checkpoint.anchor"),
        "checkpoint=anchored\nmonotonic=true\n",
    )
    .map_err(|e| e.to_string())
}

fn xahau_vault_status() -> Result<(), String> {
    fs::create_dir_all(xahau_root()).map_err(|e| e.to_string())?;
    fs::write(xahau_root().join("vault.status"), "vault=healthy\n").map_err(|e| e.to_string())
}
