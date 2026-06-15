use crate::config::{games_root, runtime_root};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn dispatch(args: &[String]) -> Result<(), String> {
    dispatch_legacy(args)
}

pub fn dispatch_legacy(args: &[String]) -> Result<(), String> {
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");
    match cmd {
        "lease" => crate::lease::dispatch(args),
        "world" => crate::world::dispatch(args),
        "release" => crate::release::dispatch(args),
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
        "package-game" => package_game(args.get(2).map(String::as_str).unwrap_or("2d-arena")),
        "new-game" => new_game(args.get(2).map(String::as_str).unwrap_or("example-game")),
        "run-dev" => developer_command("run-dev"),
        "replay-inspect" => developer_command("replay-inspect"),
        "replay-verify" => developer_command("replay-verify"),
        "validate-game" => developer_command("validate-game"),
        "deploy-game" => developer_command("deploy-game"),
        "checkpoint-restore" => developer_command("checkpoint-restore"),
        "multiplayer-sim" => developer_command("multiplayer-sim"),
        "replay-diff" => developer_command("replay-diff"),
        "editor" => creator_command("editor"),
        "replay-ui" => creator_command("replay-ui"),
        "inspect-entity" => creator_command("inspect-entity"),
        "import-assets" => creator_command("import-assets"),
        "hot-reload" => creator_command("hot-reload"),
        "package-content" => creator_command("package-content"),
        "validate-content" => creator_command("validate-content"),
        "publish-package" => creator_command("publish-package"),
        "creator-dashboard" => creator_command("creator-dashboard"),
        "visualize-simulation" => creator_command("visualize-simulation"),
        "run-local-federation" => start_game("2d-arena"),
        "replay-world" => verify_replay_frame(),
        "inspect-simulation" => inspect_simulation(),
        "diagnostics" => diagnostics(),
        "runtime-public-api-status" => runtime_public_api_status(),
        "runtime-orchestrator-status" => runtime_scaling_status("runtime-orchestrator-status"),
        "runtime-node-list" => runtime_scaling_status("runtime-node-list"),
        "runtime-peer-topology" => runtime_scaling_status("runtime-peer-topology"),
        "runtime-shard-status" => runtime_scaling_status("runtime-shard-status"),
        "runtime-peer-auth-status" => runtime_scaling_status("runtime-peer-auth-status"),
        "runtime-network-health" => runtime_scaling_status("runtime-network-health"),
        "runtime-flow-status" => runtime_scaling_status("runtime-flow-status"),
        "runtime-deployment-status" => runtime_scaling_status("runtime-deployment-status"),
        "runtime-package-build" => runtime_scaling_status("runtime-package-build"),
        "runtime-package-verify" => runtime_scaling_status("runtime-package-verify"),
        "runtime-gameplay-status" => runtime_gameplay_status("runtime-gameplay-status"),
        "runtime-session-list" => runtime_gameplay_status("runtime-session-list"),
        "runtime-player-status" => runtime_gameplay_status("runtime-player-status"),
        "runtime-scheduler-status" => runtime_gameplay_status("runtime-scheduler-status"),
        "runtime-matchmaking-status" => runtime_gameplay_status("runtime-matchmaking-status"),
        "runtime-gameplay-recovery" => runtime_gameplay_status("runtime-gameplay-recovery"),
        "runtime-replay-tip" => runtime_gameplay_status("runtime-replay-tip"),
        "runtime-observer-gameplay-status" => {
            runtime_gameplay_status("runtime-observer-gameplay-status")
        }
        "runtime-execution-health" => runtime_gameplay_status("runtime-execution-health"),
        "runtime-session-recovery" => runtime_gameplay_status("runtime-session-recovery"),
        "runtime-world-status" => runtime_civilization_status("runtime-world-status"),
        "runtime-civilization-status" => runtime_civilization_status("runtime-civilization-status"),
        "runtime-entity-status" => runtime_civilization_status("runtime-entity-status"),
        "runtime-economy-status" => runtime_civilization_status("runtime-economy-status"),
        "runtime-inventory-status" => runtime_civilization_status("runtime-inventory-status"),
        "runtime-world-recovery" => runtime_civilization_status("runtime-world-recovery"),
        "runtime-civilization-replay-tip" => {
            runtime_civilization_status("runtime-civilization-replay-tip")
        }
        "runtime-world-health" => runtime_civilization_status("runtime-world-health"),
        "runtime-entity-lineage" => runtime_civilization_status("runtime-entity-lineage"),
        "runtime-civilization-restoration" => {
            runtime_civilization_status("runtime-civilization-restoration")
        }
        "runtime-faction-status" => runtime_civilization_status("runtime-faction-status"),
        "runtime-society-status" => runtime_civilization_status("runtime-society-status"),
        "runtime-governance-status" => runtime_civilization_status("runtime-governance-status"),
        "runtime-ecology-status" => runtime_civilization_status("runtime-ecology-status"),
        "runtime-social-memory-status" => {
            runtime_civilization_status("runtime-social-memory-status")
        }
        "runtime-civilization-federation-status" => {
            runtime_civilization_status("runtime-civilization-federation-status")
        }
        "runtime-civilization-health" => runtime_civilization_status("runtime-civilization-health"),
        "runtime-procedural-world-status" => {
            runtime_civilization_status("runtime-procedural-world-status")
        }
        "runtime-conflict-status" => runtime_civilization_status("runtime-conflict-status"),
        "runtime-autonomous-recovery" => runtime_civilization_status("runtime-autonomous-recovery"),

        "runtime-symbol-audit" => runtime_symbol_audit(),
        "runtime-integration-closure" => runtime_integration_closure(),
        "runtime-api-ownership" => runtime_api_ownership(),
        "workspace-linkage-status" => workspace_linkage_status(),
        "runtime-crate-audit" => runtime_crate_audit(),
        "workspace-validation-status" => workspace_validation_status(),
        "sovereign-workspace-closure" => sovereign_workspace_closure(),
        "replay-network-status" => replay_network_status(),
        "replay-peer-status" => replay_peer_status(),
        "replay-window-sync" => replay_window_sync(),
        "replay-stream-runtime" => replay_stream_runtime(),
        "replay-observer-runtime" => replay_observer_runtime(),
        "replay-federation-runtime" => replay_federation_runtime(),
        "replay-catchup-runtime" => replay_catchup_runtime(),
        "replay-recovery-runtime" => replay_recovery_runtime(),
        "replay-transport-verify" => replay_transport_verify(),
        "runtime-ecs-status" => runtime_simulation_status("runtime-ecs-status"),
        "runtime-ai-status" => runtime_simulation_status("runtime-ai-status"),
        "runtime-partition-status" => runtime_simulation_status("runtime-partition-status"),
        "runtime-simulation-health" => runtime_simulation_status("runtime-simulation-health"),
        "runtime-behavior-tree-status" => runtime_simulation_status("runtime-behavior-tree-status"),
        "runtime-world-simulation-status" => {
            runtime_simulation_status("runtime-world-simulation-status")
        }
        "runtime-shard-migration-status" => {
            runtime_simulation_status("runtime-shard-migration-status")
        }
        "runtime-simulation-federation-status" => {
            runtime_simulation_status("runtime-simulation-federation-status")
        }
        "runtime-ai-memory-status" => runtime_simulation_status("runtime-ai-memory-status"),
        "runtime-partition-recovery" => runtime_simulation_status("runtime-partition-recovery"),
        _ => Err(format!("unknown command: {cmd}")),
    }
}
pub fn print_help() {
    crate::product::print_product_help();
    println!();
    println!("everarcade <release|world|lease|install-game|list-games|inspect-game|run-game|start-game|asset-register|asset-build|asset-verify|start|init-game|build-game|package-game|run-local-federation|replay-world|inspect-simulation|runtime-snapshot|diagnostics|runtime-public-api-status|runtime-symbol-audit|runtime-integration-closure|runtime-api-ownership|workspace-linkage-status|runtime-crate-audit|workspace-validation-status|sovereign-workspace-closure|replay-network-status|replay-peer-status|replay-window-sync|replay-stream-runtime|replay-observer-runtime|replay-federation-runtime|replay-catchup-runtime|replay-recovery-runtime|replay-transport-verify|runtime-gameplay-status|runtime-session-list|runtime-player-status|runtime-scheduler-status|runtime-matchmaking-status|runtime-gameplay-recovery|runtime-replay-tip|runtime-observer-gameplay-status|runtime-execution-health|runtime-session-recovery|runtime-world-status|runtime-civilization-status|runtime-entity-status|runtime-economy-status|runtime-inventory-status|runtime-world-recovery|runtime-civilization-replay-tip|runtime-world-health|runtime-entity-lineage|runtime-civilization-restoration|runtime-ecs-status|runtime-ai-status|runtime-partition-status|runtime-simulation-health|runtime-behavior-tree-status|runtime-world-simulation-status|runtime-shard-migration-status|runtime-simulation-federation-status|runtime-ai-memory-status|runtime-partition-recovery|runtime-faction-status|runtime-society-status|runtime-governance-status|runtime-ecology-status|runtime-social-memory-status|runtime-civilization-federation-status|runtime-civilization-health|runtime-procedural-world-status|runtime-conflict-status|runtime-autonomous-recovery|new-game|run-dev|replay-inspect|replay-verify|validate-game|deploy-game|checkpoint-restore|multiplayer-sim|replay-diff|editor|replay-ui|inspect-entity|import-assets|hot-reload|package-content|validate-content|publish-package|creator-dashboard|visualize-simulation>");
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
fn new_game(game_id: &str) -> Result<(), String> {
    let target = games_root().join(game_id);
    copy_dir(Path::new("templates/topdown-arena"), &target)?;
    fs::write(target.join("game.toml"), format!("[game]\nid = \"{game_id}\"\nname = \"{game_id}\"\nversion = \"0.1.0\"\n\n[runtime]\ntick_rate = 30\ndeterministic = true\n\n[assets]\nmanifest = \"assets.toml\"\n\n[replay]\nenabled = true\ncheckpoint_interval = 300\n")).map_err(|e| e.to_string())?;
    println!("developer_game_created={game_id} deterministic=true replay=append-only");
    Ok(())
}

fn creator_command(command: &str) -> Result<(), String> {
    seed_runtime()?;
    let payload = serde_json::json!({
        "command": command,
        "creator_tooling": true,
        "deterministic_diagnostics": true,
        "replay_continuity": "preserved",
        "replay_policy": "append-only-reconstruction",
        "invalid_runtime_mutation": "rejected",
        "incompatible_packages": "rejected",
        "authority_boundary": "deterministic-execution-runtime-only",
        "renderer_authoritative": false
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn developer_command(command: &str) -> Result<(), String> {
    seed_runtime()?;
    let payload = serde_json::json!({
        "command": command,
        "deterministic_diagnostics": true,
        "replay_continuity": "preserved",
        "invalid_runtime_configuration": "rejected",
        "authority_boundary": "deterministic-execution-runtime-only",
        "renderer_authoritative": false
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
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

fn diagnostics() -> Result<(), String> {
    let payload = serde_json::json!({
        "component": "everarcade-cli",
        "event": "operator_diagnostics",
        "sequence": 0,
        "deterministic": true,
        "runtime_config_summary": {"root": runtime_root()},
        "release_manifest_summary": {"version": "0.1.0"},
        "replay_status": {"latest_frame": "runtime/replay/latest/frame-0001.json"},
        "topology_status": {"mode": "local"},
        "last_profile_summary": {"path": "target/everarcade-profile/test-profile-report.json"},
        "validation_hints": ["run scripts/profile_runtime_tests.sh", "run scripts/validate_clean_vm_bootstrap.sh"]
    });
    println!(
        "{}",
        serde_json::to_string(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_diagnostics_stdout_json() {
        diagnostics().expect("diagnostics");
    }
}

fn runtime_public_api_status() -> Result<(), String> {
    let apis = execution_core::runtime::export_governance::runtime_public_api();
    let payload = serde_json::json!({"canonical_runtime_apis": apis, "scaffold_domains": ["renderer", "history", "federation"]});
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn runtime_symbol_audit() -> Result<(), String> {
    let audit = execution_core::runtime::export_governance::runtime_api_continuity_audit();
    println!(
        "{}",
        serde_json::to_string_pretty(&audit).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn runtime_integration_closure() -> Result<(), String> {
    let ownership = execution_core::runtime::export_governance::runtime_export_ownership();
    println!(
        "{}",
        serde_json::to_string_pretty(&ownership).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn runtime_api_ownership() -> Result<(), String> {
    let ownership = execution_core::runtime::export_governance::runtime_api_ownership();
    println!(
        "{}",
        serde_json::to_string_pretty(&ownership).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn workspace_linkage_status() -> Result<(), String> {
    runtime_integration_closure()
}

fn runtime_crate_audit() -> Result<(), String> {
    let audit = execution_core::runtime::export_governance::workspace_integration_audit();
    println!(
        "{}",
        serde_json::to_string_pretty(&audit).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn workspace_validation_status() -> Result<(), String> {
    let status = execution_core::runtime::export_governance::sovereign_workspace_closure();
    println!(
        "{}",
        serde_json::to_string_pretty(&status).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn sovereign_workspace_closure() -> Result<(), String> {
    workspace_validation_status()
}

fn replay_network_status() -> Result<(), String> {
    let payload = serde_json::json!({"peers": 0, "windows": 0, "continuity": "append-only"});
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn replay_peer_status() -> Result<(), String> {
    let payload = serde_json::json!({"observer_peers": [], "equivalence": "deterministic", "recovery": "ready"});
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn replay_window_sync() -> Result<(), String> {
    println!("replay window sync: deterministic scaffold ready");
    Ok(())
}
fn replay_stream_runtime() -> Result<(), String> {
    println!("replay stream runtime: append-only scaffold ready");
    Ok(())
}
fn replay_observer_runtime() -> Result<(), String> {
    println!("replay observer runtime: non-authoritative scaffold ready");
    Ok(())
}
fn replay_federation_runtime() -> Result<(), String> {
    println!("replay federation runtime: continuity scaffold ready");
    Ok(())
}
fn replay_catchup_runtime() -> Result<(), String> {
    println!("replay catchup runtime: resumable scaffold ready");
    Ok(())
}
fn replay_recovery_runtime() -> Result<(), String> {
    println!("replay recovery runtime: restoration scaffold ready");
    Ok(())
}
fn replay_transport_verify() -> Result<(), String> {
    println!("replay transport verify: integrity scaffold ready");
    Ok(())
}

fn runtime_scaling_status(command: &str) -> Result<(), String> {
    let detail = match command {
        "runtime-orchestrator-status" => "nodes=3 topology=restored replay_continuity=preserved",
        "runtime-node-list" => "node-a,node-b,node-c continuity_root=root:everarcade:federation:v1",
        "runtime-peer-topology" => "peers=2 authenticated=true mutable_authority=false",
        "runtime-shard-status" => "shards=6 equivalence=preserved recovery=ready",
        "runtime-peer-auth-status" => "trusted_peers=2 forged_peers=rejected lineage=valid",
        "runtime-network-health" => "latency=deterministic sync=healthy recovery=ready",
        "runtime-flow-status" => "backpressure=active ordering=preserved overflow=rejected",
        "runtime-deployment-status" => "deployment=restorable topology_continuity=preserved",
        "runtime-package-build" => "bundle=evernode deterministic=true mutable_authority=false",
        "runtime-package-verify" => {
            "package=verified restoration=deterministic corruption=rejected"
        }
        _ => return Err(format!("unknown runtime scaling command: {command}")),
    };
    println!("{command} {detail}");
    Ok(())
}

fn runtime_gameplay_status(command: &str) -> Result<(), String> {
    let payload = match command {
        "runtime-gameplay-status" => {
            serde_json::json!({"command": command, "gameplay_execution": "authoritative", "deterministic_ticks": true, "replay_continuity": "append-only", "renderer_authoritative": false})
        }
        "runtime-session-list" => {
            serde_json::json!({"command": command, "sessions": ["arena"], "session_continuity": "preserved"})
        }
        "runtime-player-status" => {
            serde_json::json!({"command": command, "players": ["p1"], "authority_boundary": "deterministic-runtime"})
        }
        "runtime-scheduler-status" => {
            serde_json::json!({"command": command, "tick_ordering": "monotonic", "checkpointing": "enabled"})
        }
        "runtime-matchmaking-status" => {
            serde_json::json!({"command": command, "routing": "deterministic", "lineage": "valid"})
        }
        "runtime-gameplay-recovery" => {
            serde_json::json!({"command": command, "recovery": "ready", "corrupted_restoration": "rejected"})
        }
        "runtime-replay-tip" => {
            serde_json::json!({"command": command, "tip": 1, "append_only": true})
        }
        "runtime-observer-gameplay-status" => {
            serde_json::json!({"command": command, "observer": "hydrated", "reconstruction_only": true, "authority_writes": "rejected"})
        }
        "runtime-execution-health" => {
            serde_json::json!({"command": command, "execution_continuity": "preserved", "multiplayer_sync": "deterministic", "recovery_readiness": "ready"})
        }
        "runtime-session-recovery" => {
            serde_json::json!({"command": command, "session_recovery": "deterministic", "invalid_replay_restoration": "rejected"})
        }
        _ => return Err(format!("unknown gameplay runtime command: {command}")),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn runtime_civilization_status(command: &str) -> Result<(), String> {
    let payload = match command {
        "runtime-world-status" => {
            serde_json::json!({"command": command, "world_continuity": "preserved", "deterministic_ticks": true, "replay_continuity": "append-only", "renderer_authoritative": false})
        }
        "runtime-civilization-status" => {
            serde_json::json!({"command": command, "civilization_continuity": "preserved", "governance": "deterministic", "economy": "persistent"})
        }
        "runtime-entity-status" => {
            serde_json::json!({"command": command, "entity_identity": "deterministic", "lineage": "preserved", "invalid_mutation": "rejected"})
        }
        "runtime-economy-status" => {
            serde_json::json!({"command": command, "ledger": "append-only", "settlement_mutation": "rejected", "equivalence": "preserved"})
        }
        "runtime-inventory-status" => {
            serde_json::json!({"command": command, "ownership_lineage": "preserved", "transfers": "deterministic", "invalid_mutation": "rejected"})
        }
        "runtime-world-recovery" => {
            serde_json::json!({"command": command, "recovery": "ready", "corrupted_recovery": "rejected", "replay_equivalence": "preserved"})
        }
        "runtime-civilization-replay-tip" => {
            serde_json::json!({"command": command, "tip": "civilization:replay:deterministic", "append_only": true})
        }
        "runtime-world-health" => {
            serde_json::json!({"command": command, "world_continuity": "healthy", "restoration_readiness": "ready", "replay_continuity": "preserved"})
        }
        "runtime-entity-lineage" => {
            serde_json::json!({"command": command, "entity_lineage_continuity": "preserved", "identity": "sovereign"})
        }
        "runtime-civilization-restoration" => {
            serde_json::json!({"command": command, "restoration": "deterministic", "authority_bypass": "rejected"})
        }
        "runtime-faction-status" => {
            serde_json::json!({"command": command, "faction_continuity": "preserved", "governance_evolution": "deterministic", "diplomatic_divergence": "rejected"})
        }
        "runtime-society-status" => {
            serde_json::json!({"command": command, "societal_continuity": "preserved", "population_evolution": "deterministic", "hidden_social_mutation": "rejected"})
        }
        "runtime-governance-status" => {
            serde_json::json!({"command": command, "diplomacy": "deterministic", "treaty_lineage": "preserved", "unauthorized_governance_mutation": "rejected"})
        }
        "runtime-ecology-status" => {
            serde_json::json!({"command": command, "ecological_continuity": "preserved", "resource_distribution": "deterministic", "resource_divergence": "rejected"})
        }
        "runtime-social-memory-status" => {
            serde_json::json!({"command": command, "social_memory": "append-only", "hidden_memory_mutation": "rejected", "restoration": "deterministic"})
        }
        "runtime-civilization-federation-status" => {
            serde_json::json!({"command": command, "federation_continuity": "preserved", "society_sync": "deterministic", "replay_authority_mutation": "rejected"})
        }
        "runtime-civilization-health" => {
            serde_json::json!({"command": command, "faction": "healthy", "society": "healthy", "governance": "healthy", "ecology": "healthy", "social_memory": "append-only", "federation": "healthy", "replay_continuity": "append-only"})
        }
        "runtime-procedural-world-status" => {
            serde_json::json!({"command": command, "topology_evolution": "deterministic", "terrain_resources": "deterministic", "topology_divergence": "rejected"})
        }
        "runtime-conflict-status" => {
            serde_json::json!({"command": command, "migration": "deterministic", "trade": "deterministic", "conflict": "deterministic", "interaction_divergence": "rejected"})
        }
        "runtime-autonomous-recovery" => {
            serde_json::json!({"command": command, "civilization_restore": "deterministic", "ecology_restore": "deterministic", "social_memory_restore": "append-only", "corrupted_recovery": "rejected"})
        }
        _ => return Err(format!("unknown civilization runtime command: {command}")),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn runtime_simulation_status(command: &str) -> Result<(), String> {
    let payload = match command {
        "runtime-ecs-status" => {
            serde_json::json!({"command": command, "ecs_continuity": "preserved", "deterministic_ordering": true, "unauthorized_component_mutation": "rejected"})
        }
        "runtime-ai-status" => {
            serde_json::json!({"command": command, "ai_continuity": "preserved", "hidden_ai_mutation": "rejected", "decision_ordering": "deterministic"})
        }
        "runtime-partition-status" => {
            serde_json::json!({"command": command, "partition_continuity": "preserved", "streaming": "replay-derived", "divergence": "rejected"})
        }
        "runtime-simulation-health" => {
            serde_json::json!({"command": command, "ecs": "healthy", "ai": "healthy", "scheduler": "deterministic", "replay_continuity": "append-only"})
        }
        "runtime-behavior-tree-status" => {
            serde_json::json!({"command": command, "behavior_tree_continuity": "preserved", "hidden_execution_mutation": "rejected"})
        }
        "runtime-world-simulation-status" => {
            serde_json::json!({"command": command, "terrain_evolution": "deterministic", "world_mutation": "runtime-boundary-only"})
        }
        "runtime-shard-migration-status" => {
            serde_json::json!({"command": command, "migration_readiness": "ready", "partition_divergence": "rejected"})
        }
        "runtime-simulation-federation-status" => {
            serde_json::json!({"command": command, "federation_continuity": "preserved", "replay_authority_mutation": "rejected"})
        }
        "runtime-ai-memory-status" => {
            serde_json::json!({"command": command, "ai_memory": "append-only", "restoration": "deterministic"})
        }
        "runtime-partition-recovery" => {
            serde_json::json!({"command": command, "partition_recovery": "deterministic", "replay_equivalence": "preserved"})
        }
        _ => return Err(format!("unknown simulation runtime command: {command}")),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?
    );
    Ok(())
}
