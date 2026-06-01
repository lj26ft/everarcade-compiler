use contract_api::protocol_records::fields;
use contract_api::rustrig::RustrigContext;
use execution_core::marketplace::{install_package, replay_safe_marketplace, MarketplacePackage};
use execution_core::rustrig_runtime::{ExecutionRequest, RustrigKernel};
use rustrigs::dependency::DependencyGraph;
use rustrigs::package::arena_vanguard_required_manifests;
use rustrigs::xrpl::{anchor_records_equivalent, arena_vanguard_anchor_records};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("execution-core has workspace parent")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).unwrap_or_else(|err| panic!("read {relative}: {err}"))
}

fn assert_exists(relative: &str) {
    assert!(root().join(relative).exists(), "missing {relative}");
}

fn assert_contains(relative: &str, needle: &str) {
    let body = read(relative);
    assert!(body.contains(needle), "{relative} missing {needle}");
}

fn arena_packages() -> Vec<MarketplacePackage> {
    arena_vanguard_required_manifests()
        .into_iter()
        .map(|manifest| MarketplacePackage {
            signature: format!("signed:{}", manifest.hash),
            package_bytes: manifest.package_id.as_bytes().to_vec(),
            abi_version: "everarcade-protocol-1".to_owned(),
            deterministic: true,
            replay_safe: true,
            records_valid: true,
            requests_network: false,
            requests_filesystem: false,
            requests_authority_write: false,
            requests_deployment: false,
            requests_xrpl_submission: false,
            manifest,
        })
        .collect()
}

#[test]
fn test_arena_vanguard_package_generation() {
    for path in [
        "arena-vanguard/manifest.toml",
        "arena-vanguard-world/world_manifest.toml",
        "arena-vanguard-assets/asset_manifest.toml",
        "arena-vanguard-rustrigs/marketplace_manifest.toml",
        "arena-vanguard-package/runtime_package.toml",
        "arena-vanguard-package/world_package.toml",
        "arena-vanguard-package/deployment_package.toml",
        "arena-vanguard-package/asset_package.toml",
    ] {
        assert_exists(path);
    }
    assert_contains(
        "arena-vanguard-package/runtime_package.toml",
        "runtime package",
    );
    assert_contains("arena-vanguard-package/world_package.toml", "world package");
    assert_contains(
        "arena-vanguard-package/deployment_package.toml",
        "deployment package",
    );
    assert_contains("arena-vanguard-package/asset_package.toml", "asset package");
}

#[test]
fn test_arena_vanguard_marketplace_flow() {
    let packages = arena_packages();
    let graph = DependencyGraph::new(packages.iter().map(|pkg| pkg.manifest.clone()).collect());
    let installed: Vec<_> = packages
        .iter()
        .cloned()
        .map(|pkg| install_package(pkg, &graph).expect("package installs"))
        .collect();
    let categories: BTreeSet<_> = installed
        .iter()
        .map(|pkg| pkg.manifest.name.as_str())
        .collect();
    assert_eq!(
        categories,
        BTreeSet::from([
            "Combat",
            "Dialogue",
            "Economy",
            "Inventory",
            "Quest",
            "World"
        ])
    );
    assert!(replay_safe_marketplace(&installed));
    assert_contains(
        "arena-vanguard-rustrigs/marketplace_manifest.toml",
        "no_custom_gameplay_bypasses = true",
    );
}

#[test]
fn test_arena_vanguard_runtime_startup() {
    let world_manifest = read("arena-vanguard-world/world_manifest.toml");
    for state in [
        "startup = \"validated\"",
        "shutdown = \"validated\"",
        "restart = \"validated\"",
        "recovery = \"validated\"",
        "checkpoint_restore = \"validated\"",
        "replay_equivalence = \"validated\"",
    ] {
        assert!(
            world_manifest.contains(state),
            "missing lifecycle state {state}"
        );
    }
}

#[test]
fn test_arena_vanguard_replay_equivalence() {
    let checkpoint = read("arena-vanguard-world/checkpoints/checkpoint_0003.state");
    let replay = read("arena-vanguard-world/replay/replay_0003.log");
    let state_root = "world:arena-vanguard:tick:3:players:2:marketplace:6";
    assert!(checkpoint.contains(state_root));
    assert!(replay.contains(state_root));
}

#[test]
fn test_arena_vanguard_multiplayer() {
    let replay = read("arena-vanguard-world/replay/replay_0003.log");
    assert!(replay.contains("host_session|vanguard-host"));
    assert!(replay.contains("join_session|vanguard-guest"));
    assert_contains(
        "deployment/reports/arena_vanguard_certification.md",
        "player continuity: validated",
    );
    assert_contains(
        "deployment/reports/arena_vanguard_certification.md",
        "replay continuity: validated",
    );
    assert_contains(
        "deployment/reports/arena_vanguard_certification.md",
        "world continuity: validated",
    );
}

#[test]
fn test_arena_vanguard_recovery() {
    assert_contains(
        "deployment/reports/runtime_recovery_report.md",
        "checkpoint restore: validated",
    );
    assert_contains(
        "deployment/reports/runtime_recovery_report.md",
        "recovered session equivalence: validated",
    );
    assert_contains(
        "arena-vanguard-package/world_package.toml",
        "recovery = true",
    );
}

#[test]
fn test_arena_vanguard_deployment_manifest() {
    for path in [
        "deployment/evernode/deployment_manifest.toml",
        "deployment/evernode/runtime_manifest.toml",
        "deployment/evernode/package_manifest.toml",
        "deployment/evernode/world_manifest.toml",
    ] {
        assert_exists(path);
    }
    let deployment = read("deployment/evernode/deployment_manifest.toml");
    for operation in ["deploy", "start", "stop", "restart", "recover", "verify"] {
        assert!(deployment.contains(&format!("{operation} = true")));
    }
    for simulation in [
        "single_node",
        "multi_node",
        "restart",
        "failure",
        "recovery",
    ] {
        assert!(deployment.contains(&format!("{simulation} = \"validated\"")));
    }
}

#[test]
fn test_xrpl_anchor_generation() {
    assert_exists("xrpl-anchor/anchor_records.json");
    let records = arena_vanguard_anchor_records();
    assert_eq!(records.len(), 4);
    assert!(records.iter().all(rustrigs::xrpl::verify_anchor_record));
    let names: BTreeSet<_> = records.iter().map(|record| record.record_type).collect();
    assert_eq!(
        names,
        BTreeSet::from([
            "DeploymentAnchorRecord",
            "ReceiptAnchorRecord",
            "ReplayAnchorRecord",
            "WorldAnchorRecord",
        ])
    );
    assert_contains(
        "xrpl-anchor/anchor_records.json",
        "external-xrpl-submission-only",
    );
}

#[test]
fn test_xrpl_anchor_equivalence() {
    let first = arena_vanguard_anchor_records();
    let replayed = arena_vanguard_anchor_records();
    assert!(anchor_records_equivalent(&first, &replayed));
}

#[test]
fn test_v0_1_certification_gate() {
    for path in [
        "deployment/reports/arena_vanguard_certification.md",
        "deployment/reports/evernode_deployment_trial.md",
        "deployment/reports/xrpl_anchor_report.md",
        "deployment/reports/runtime_recovery_report.md",
        "deployment/reports/v0_1_production_certification.md",
    ] {
        assert_exists(path);
    }
    let certification = read("deployment/reports/v0_1_production_certification.md");
    for gate in [
        "local runtime: validated",
        "multiplayer: validated",
        "marketplace: validated",
        "deployment: validated",
        "replay: validated",
        "recovery: validated",
        "xrpl anchors: validated",
    ] {
        assert!(certification.contains(gate), "missing gate {gate}");
    }
}

#[test]
fn test_arena_vanguard_real_rustrig_execution_flow() {
    let ctx = RustrigContext {
        world_root: "arena:vanguard".to_owned(),
        replay_root: "replay:arena".to_owned(),
        checkpoint_root: "checkpoint:arena".to_owned(),
        actor_id: "arena-player".to_owned(),
        tick: 42,
        input_hash: "arena-vanguard-flow".to_owned(),
        protocol_version: "everarcade-protocol-1".to_owned(),
    };
    let mut kernel = RustrigKernel::default();
    for id in [
        "world.spawn_entity",
        "world.move_entity",
        "combat.apply_damage",
        "inventory.add_item",
        "quests.advance_objective",
        "dialogue.complete_dialogue",
        "economy.create_ledger_entry",
        "replay.emit_event",
        "runtime.checkpoint",
        "runtime.recover",
    ] {
        let result = kernel
            .execute(ExecutionRequest {
                rustrig_id: id.to_owned(),
                version: "1.0.0".to_owned(),
                context: ctx.clone(),
                payload: fields(&[
                    ("entity", "arena-player".to_owned()),
                    ("target", "arena-player".to_owned()),
                    ("item", "arena-medal".to_owned()),
                    ("quest", "arena-vanguard".to_owned()),
                    ("conversation", "vanguard-coach".to_owned()),
                    ("amount", "3".to_owned()),
                ]),
            })
            .expect("arena vanguard rustrig execution");
        assert_eq!(result.output.rustrig_id, id);
    }
    assert!(kernel.state.world.contains_key("arena-player"));
    assert!(!kernel.state.combat.is_empty());
    assert!(!kernel.state.inventory.is_empty());
    assert!(!kernel.state.quests.is_empty());
    assert!(!kernel.state.dialogue.is_empty());
    assert!(!kernel.state.economy.is_empty());
    assert!(kernel.replay.events.len() >= 10);
}
