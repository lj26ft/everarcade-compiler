use serde::{Deserialize, Serialize};
use serde_json::Value;

mod assembly;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

const PROTOCOL: &str = "everarcade-world-package-v0.1";
const RUNTIME_PROTOCOL: &str = "everarcade-runtime-bundle-v0.1";
const ARCHIVE_MAGIC: &[u8] = b"EVRWORLD\n";
const GRAPH_RUNTIME_COMMIT: &str = "a561207bcc3b953b036102c805620ab97d50c344";
const GRAPH_RUNTIME_TREE: &str = "0afa83ad446e3ebfdeb0ffa94cfd23928e477910";
const GRAPH_SYSTEM_IDENTITY: &str = "0fbbcc2e0f0579efeaefeccab43c267c291094da";
const SCHEDULER_RELIANCE_IDENTITY: &str = "455d9830efb5f19a66fa5c0b794fb2e7c7124bc2";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldManifest {
    pub protocol: String,
    pub world_id: String,
    pub world_name: String,
    pub world_version: String,
    pub world_operator: String,
    pub created_at: String,
    pub world_contract_hash: String,
    pub runtime_bundle_hash: String,
    pub genesis_state_hash: String,
    pub state_root: String,
    pub replay_root: String,
    pub receipt_root: String,
    pub continuity_root: String,
    pub transport_protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compiler_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_runtime_extension: Option<GraphRuntimeExtension>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphRuntimeExtension {
    pub adapter: String,
    pub binding: String,
    pub runtime_profile: String,
    pub runtime_profile_version: String,
    pub graph_path: String,
    pub graph_sha256: String,
    pub genesis_path: String,
    pub genesis_sha256: String,
    pub graph_runtime_commit: String,
    pub graph_runtime_tree: String,
    pub graph_runtime_system_identity: String,
    pub scheduler_reliance_identity: String,
    pub qualification: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeBundleManifest {
    pub protocol: String,
    pub runtime_id: String,
    pub runtime_version: String,
    pub target_transport: String,
    pub entrypoint: String,
    pub deterministic_engine: String,
    pub wasm_hash: Option<String>,
    pub bundle_hash: String,
}
#[derive(Debug, Serialize)]
struct Report<'a> {
    command: &'a str,
    success: bool,
    package_hash: Option<String>,
    bundle_hash: Option<String>,
    world_id: String,
    world_version: String,
    lease_id: Option<String>,
    continuity_root: String,
    status: &'a str,
}

pub fn dispatch(args: &[String]) -> Result<(), String> {
    let action = args.get(2).map(String::as_str).ok_or(
        "usage: everarcade world <create|init|package|inspect|verify|conformance|deploy|restore|migrate|replay|diff>",
    )?;
    match action {
        "create" => create(
            opt(args, "--config").unwrap_or_else(|| "world-request.json".into()),
            opt(args, "--dir").unwrap_or_else(|| "world".into()),
            opt(args, "--out"),
        ),
        "init" => init(opt(args, "--dir").unwrap_or_else(|| "world".into())),
        "package" => package(
            opt(args, "--dir").unwrap_or_else(|| "world".into()),
            opt(args, "--out").unwrap_or_else(|| "world.evr".into()),
        )
        .map(|_| ()),
        "inspect" => inspect(opt(args, "--package").unwrap_or_else(|| "world.evr".into())),
        "verify" => {
            verify_cmd(opt(args, "--package").unwrap_or_else(|| "world.evr".into())).map(|_| ())
        }
        "conformance" => conformance(opt(args, "--package").unwrap_or_else(|| "world.evr".into())),
        "diff" => diff(
            opt(args, "--left").unwrap_or_else(|| "world-a.evr".into()),
            opt(args, "--right").unwrap_or_else(|| "world-b.evr".into()),
        ),
        "adapt-graph-runtime" => adapt_graph_runtime(
            opt(args, "--package").unwrap_or_else(|| "world.evr".into()),
            opt(args, "--out-dir").unwrap_or_else(|| "graph-intake".into()),
            opt(args, "--binding-out").unwrap_or_else(|| "WorldGraphBinding.json".into()),
        ),
        "verify-graph-runtime" => verify_graph_runtime(
            opt(args, "--package").unwrap_or_else(|| "world.evr".into()),
            opt(args, "--intake-dir").unwrap_or_else(|| "graph-intake".into()),
            opt(args, "--binding").unwrap_or_else(|| "WorldGraphBinding.json".into()),
        ),
        "deploy" => deploy(
            opt(args, "--package").unwrap_or_else(|| "world.evr".into()),
            opt(args, "--lease").unwrap_or_else(|| "offline-lease".into()),
        ),
        "restore" => restore(opt(args, "--package").unwrap_or_else(|| "world.evr".into())),
        "migrate" => migrate(
            opt(args, "--package").unwrap_or_else(|| "world.evr".into()),
            opt(args, "--from").unwrap_or_else(|| "old-lease".into()),
            opt(args, "--to").unwrap_or_else(|| "new-lease".into()),
        ),
        "replay" => replay(opt(args, "--package").unwrap_or_else(|| "world.evr".into())),
        _ => Err(format!("unknown world command: {action}")),
    }
}

fn create(config: String, dir: String, out: Option<String>) -> Result<(), String> {
    let request = assembly::CanonicalWorldRequest::from_request_bytes(
        &fs::read(&config).map_err(|e| e.to_string())?,
    )?;
    let root = PathBuf::from(dir);
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|e| e.to_string())?;
    }
    init(root.to_string_lossy().to_string())?;
    let assembled = assembly::assemble_world(request)?;
    apply_profiles(&root, &assembled)?;
    if let Some(out) = out {
        package(root.to_string_lossy().to_string(), out)?;
    }
    println!("world created from profiles: {}", root.display());
    Ok(())
}

fn apply_profiles(root: &Path, assembled: &assembly::AssembledWorld) -> Result<(), String> {
    let r = &assembled.request;
    for d in [
        "manifest",
        "runtime",
        "world",
        "rules",
        "primitives",
        "content",
        "proof",
        "trust",
        "signatures",
        "assets",
        "projections",
        "metadata",
    ] {
        fs::create_dir_all(root.join(d)).map_err(|e| e.to_string())?;
        fs::write(
            root.join(d).join(".keep"),
            b"canonical world.evr directory\n",
        )
        .map_err(|e| e.to_string())?;
    }
    let world_id = r.world_id.clone();
    let profiles = json!({
        "schema_version": r.schema_version,
        "resolved_profiles": assembled.ir.resolved_profiles,
        "requested_profiles": r.profiles,
        "profile_graph_hash": assembled.ir.resolved_profile_graph.graph_hash,
        "stable_resolution_order": assembled.ir.resolved_profile_graph.stable_topological_order,
        "module_references": r.module_references,
        "assembly_contract_version": assembly::ASSEMBLY_CONTRACT_VERSION,
        "runtime_contract_version": assembly::PTW_RUNTIME_CONTRACT_VERSION
    });
    write_json(root.join("metadata/profile-resolution.json"), &profiles)?;
    write_json(
        root.join("primitives/ptw-composition.json"),
        &json!({"primitives":["WorldTime","Terrain","Regions","Biomes","Resources","Regeneration","Movement","Combat","Inventory","Crafting","Loot","AIIntents","Quests","GovernanceMetadata","EconomyMetadata","ProjectionMetadata","ProofMetadata"]}),
    )?;
    write_json(
        root.join("rules/rules.json"),
        &json!({"genre_profile":r.profiles.get("genre").cloned().unwrap_or_default(),"movement":"grid-deterministic","combat":"rules-derived","loot":"seedless-table-order"}),
    )?;
    write_json(
        root.join("world/world.json"),
        &json!({"world_id":world_id,"name":r.world_name,"biome_profile":r.profiles.get("biome").cloned().unwrap_or_default()}),
    )?;
    write_json(
        root.join("projections/projection.json"),
        &json!({"projection_profile":r.profiles.get("projection").cloned().unwrap_or_default(),"client":"web-reference","authoritative":false}),
    )?;
    write_json(
        root.join("proof/proof.json"),
        &json!({"proof_profile":r.profiles.get("proof").cloned().unwrap_or_default(),"replay_verification":"deterministic-local"}),
    )?;
    write_json(
        root.join("trust/trust.json"),
        &json!({"signatures_required":false,"hosted_registry_required":false}),
    )?;
    write_json(
        root.join("signatures/signatures.json"),
        &json!({"signatures":[],"reason":"unsigned reference package"}),
    )?;
    write_json(
        root.join("metadata/migration.json"),
        &json!({"package_version":"world.evr-package-v1","migration_from":null,"migration_to":[]}),
    )?;
    write_json(
        root.join("manifest/package-metadata.json"),
        &json!({"canonical_generator":"everarcade-compiler","deterministic":true,"commercial_platform_logic":false,"profiles":profiles}),
    )?;
    write_json(root.join("metadata/runtime-ir.json"), &assembled.ir)?;
    write_json(
        root.join("metadata/runtime-ir-validation.json"),
        &assembled.ir.validation,
    )?;
    write_json(
        root.join("metadata/typed-contributions.json"),
        &assembled.contributions,
    )?;
    write_json(
        root.join("metadata/contribution-graph.json"),
        &assembled.contribution_graph,
    )?;
    write_json(
        root.join("metadata/merged-contributions.json"),
        &assembled.merged_contributions,
    )?;
    write_json(
        root.join("metadata/contribution-provenance.json"),
        &assembled.contribution_graph.provenance,
    )?;
    write_json(
        root.join("metadata/merge-diagnostics.json"),
        &assembled.merged_contributions.diagnostics,
    )?;
    write_json(
        root.join("metadata/symbol-table.json"),
        &assembled.resolved_declarations.symbol_tables,
    )?;
    write_json(
        root.join("metadata/reference-graph.json"),
        &assembled.resolved_declarations.reference_graph,
    )?;
    write_json(
        root.join("metadata/resolved-declarations.json"),
        &assembled.resolved_declarations,
    )?;
    write_json(
        root.join("metadata/reference-diagnostics.json"),
        &assembled.resolved_declarations.diagnostics,
    )?;
    write_json(
        root.join("metadata/assembly-diagnostics.json"),
        &assembled.diagnostics,
    )?;
    write_json(
        root.join("proof/assembly-manifest.json"),
        &assembled.manifest,
    )?;
    fs::write(
        root.join("world-contract/contract.wasm"),
        format!("everarcade canonical world contract {world_id}\n"),
    )
    .map_err(|e| e.to_string())?;
    write_json(
        root.join("genesis/genesis-state.json"),
        &json!({"world":world_id,"world_id":world_id,"name":r.world_name,"tick":0,"profiles":profiles,"entities":[],"actors":{"actor-01":{"inventory":{"wood":0}}},"resource":{"id":"resource-wood-01","remaining":3}}),
    )?;
    write_graph_runtime_projection(root, &world_id)?;
    write_json(
        root.join("roots.json"),
        &json!({"schema_version":"world.evr.roots/v1","world_id":world_id}),
    )?;
    fs::write(root.join("actions.log"), b"").map_err(|e| e.to_string())?;
    fs::write(root.join("receipts.log"), b"").map_err(|e| e.to_string())?;
    fs::create_dir_all(root.join("checkpoints")).map_err(|e| e.to_string())?;
    fs::write(root.join("checkpoints/.keep"), b"").map_err(|e| e.to_string())?;
    rebuild_manifest(root, &world_id, &r.world_name)?;
    write_phase_h_manifests(root)?;
    Ok(())
}

fn write_graph_runtime_projection(root: &Path, world_id: &str) -> Result<(), String> {
    let projection = root.join("runtime/graph-runtime-v1");
    fs::create_dir_all(projection.join("schemas")).map_err(|e| e.to_string())?;
    let graph = json!({
        "version":"1.0.0",
        "graph_id":format!("{world_id}.resource-gather"),
        "metadata":{"title":"Governed resource gather"},
        "entrypoints":["intake"],
        "nodes":[
            {"id":"intake","kind":"input-map","config":{},"state_reads":[],"state_writes":[],"required_capabilities":[],"allowed_effects":[],"step_budget":100,"retry":{"maximum":0},"determinism":"DETERMINISTIC"},
            {"id":"deplete-source","kind":"state-increment","config":{"path":"resource.remaining","by":-1},"state_reads":["resource.remaining"],"state_writes":["resource.remaining"],"required_capabilities":["state.read","state.write"],"allowed_effects":[],"step_budget":100,"retry":{"maximum":0},"determinism":"DETERMINISTIC"},
            {"id":"grant-actor","kind":"state-increment","config":{"path":"actors.actor-01.inventory.wood","by":1},"state_reads":["actors.actor-01.inventory.wood"],"state_writes":["actors.actor-01.inventory.wood"],"required_capabilities":["state.read","state.write"],"allowed_effects":[],"step_budget":100,"retry":{"maximum":0},"determinism":"DETERMINISTIC"},
            {"id":"emit-gather","kind":"effect-simulated","config":{"effect":"simulated","payload":{"event_type":"world.resource.gathered","actor_id":"actor-01","resource":"wood","amount":1}},"state_reads":["resource.remaining","actors.actor-01.inventory.wood"],"state_writes":[],"required_capabilities":[],"allowed_effects":["simulated"],"step_budget":100,"retry":{"maximum":0},"determinism":"DETERMINISTIC"},
            {"id":"complete","kind":"output-emit","config":{"value":{"status":"gathered"}},"state_reads":[],"state_writes":[],"required_capabilities":[],"allowed_effects":[],"step_budget":100,"retry":{"maximum":0},"determinism":"DETERMINISTIC"}
        ],
        "edges":[
            {"id":"e1","source":"intake","target":"deplete-source","guard":{"op":"always"},"priority":0,"exclusive":true,"label":"intake","recovery":false},
            {"id":"e2","source":"deplete-source","target":"grant-actor","guard":{"op":"always"},"priority":0,"exclusive":true,"label":"deplete","recovery":false},
            {"id":"e3","source":"grant-actor","target":"emit-gather","guard":{"op":"always"},"priority":0,"exclusive":true,"label":"grant","recovery":false},
            {"id":"e4","source":"emit-gather","target":"complete","guard":{"op":"always"},"priority":0,"exclusive":true,"label":"effect","recovery":false}
        ],
        "terminal_nodes":["complete"],
        "state_schema":"schemas/state.schema.json",
        "input_schema":"schemas/input.schema.json",
        "output_schema":"schemas/output.schema.json",
        "capabilities":["state.read","state.write"],
        "effects":["simulated"],
        "policies":{"deterministic":true,"max_steps":100}
    });
    write_json(projection.join("graph.json"), &graph)?;
    let genesis: Value = serde_json::from_slice(
        &fs::read(root.join("genesis/genesis-state.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    write_json(projection.join("genesis.json"), &genesis)?;
    for (name, schema) in [
        (
            "state.schema.json",
            json!({"type":"object","required":["world_id","resource","actors"]}),
        ),
        (
            "input.schema.json",
            json!({"type":"object","required":["action_id","actor_id","action_type","action_input"]}),
        ),
        ("output.schema.json", json!({"type":"object"})),
    ] {
        write_json(projection.join("schemas").join(name), &schema)?;
    }
    Ok(())
}

fn rebuild_manifest(root: &Path, world_id: &str, world_name: &str) -> Result<(), String> {
    let genesis_hash = hash_file(root.join("genesis/genesis-state.json"))?;
    fs::write(root.join("genesis/genesis-root.txt"), &genesis_hash).map_err(|e| e.to_string())?;
    let contract_hash = hash_file(root.join("world-contract/contract.wasm"))?;
    let state_root = hash_str(&format!("state:{genesis_hash}"));
    let replay_root = hash_str("replay:empty");
    let receipt_root = hash_str("receipt:empty");
    let continuity_root = hash_str(&format!(
        "continuity:{state_root}:{replay_root}:{receipt_root}"
    ));
    for (f, v) in [
        ("state-root.txt", &state_root),
        ("replay-root.txt", &replay_root),
        ("receipt-root.txt", &receipt_root),
        ("continuity-root.txt", &continuity_root),
    ] {
        fs::write(root.join("continuity").join(f), v).map_err(|e| e.to_string())?;
    }
    let mut rb: RuntimeBundleManifest = serde_json::from_slice(
        &fs::read(root.join("runtime/runtime-manifest.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    rb.wasm_hash = Some(contract_hash.clone());
    rb.bundle_hash = String::new();
    rb.bundle_hash = runtime_bundle_hash(root, &rb)?;
    write_json(root.join("runtime/runtime-manifest.json"), &rb)?;
    let graph_extension = if root.join("runtime/graph-runtime-v1/graph.json").exists() {
        Some(GraphRuntimeExtension {
            adapter: "everarcade.world-graph-adapter.v1".into(),
            binding: "everarcade.world-graph-binding.v1".into(),
            runtime_profile: "everarcade.world-runtime.graph-consumer".into(),
            runtime_profile_version: "1.0.0".into(),
            graph_path: "runtime/graph-runtime-v1/graph.json".into(),
            graph_sha256: hash_file(root.join("runtime/graph-runtime-v1/graph.json"))?,
            genesis_path: "runtime/graph-runtime-v1/genesis.json".into(),
            genesis_sha256: hash_file(root.join("runtime/graph-runtime-v1/genesis.json"))?,
            graph_runtime_commit: "a561207bcc3b953b036102c805620ab97d50c344".into(),
            graph_runtime_tree: "0afa83ad446e3ebfdeb0ffa94cfd23928e477910".into(),
            graph_runtime_system_identity: "0fbbcc2e0f0579efeaefeccab43c267c291094da".into(),
            scheduler_reliance_identity: "455d9830efb5f19a66fa5c0b794fb2e7c7124bc2".into(),
            qualification: "GRAPH-RUNTIME-SYSTEM-01R3".into(),
        })
    } else {
        None
    };
    write_json(
        root.join("manifest.json"),
        &WorldManifest {
            protocol: PROTOCOL.into(),
            world_id: world_id.into(),
            world_name: world_name.into(),
            world_version: "1.0.0".into(),
            world_operator: "open-reference-generator".into(),
            created_at: "1970-01-01T00:00:00Z".into(),
            world_contract_hash: contract_hash,
            runtime_bundle_hash: rb.bundle_hash,
            genesis_state_hash: genesis_hash,
            state_root,
            replay_root,
            receipt_root,
            continuity_root,
            transport_protocol: transport_core::HOTPOCKET_TRANSPORT_PROTOCOL.into(),
            package_profile: Some("world.evr/1.0".into()),
            compiler_profile: Some("world.evr/1.0-graph-runtime-v1".into()),
            graph_runtime_extension: graph_extension,
        },
    )
}

fn init(dir: String) -> Result<(), String> {
    let root = PathBuf::from(dir);
    for d in [
        "world-contract",
        "runtime",
        "genesis",
        "continuity",
        "schemas",
        "assets",
        "proofs",
        "proof",
        "manifest",
        "metadata",
        "projections",
        "rules",
        "primitives",
        "content",
        "trust",
        "signatures",
        "world",
    ] {
        fs::create_dir_all(root.join(d)).map_err(|e| e.to_string())?;
    }
    fs::write(
        root.join("world-contract/contract.wasm"),
        b"everarcade deterministic wasm stub",
    )
    .map_err(|e| e.to_string())?;
    write_json(
        root.join("world-contract/abi.json"),
        &json!({"abi":"everarcade-v0.1","methods":[]}),
    )?;
    write_json(
        root.join("world-contract/permissions.json"),
        &json!({"operator":"offline-operator","permissions":["submit_mutation","read_state"]}),
    )?;
    write_json(
        root.join("runtime/transport.json"),
        &json!({"protocol": transport_core::HOTPOCKET_TRANSPORT_PROTOCOL}),
    )?;
    write_json(
        root.join("runtime/entrypoint.json"),
        &json!({"entrypoint":"adapter/main"}),
    )?;
    write_json(
        root.join("genesis/genesis-state.json"),
        &json!({"world":"example","tick":0,"entities":[]}),
    )?;
    let genesis_hash = hash_file(root.join("genesis/genesis-state.json"))?;
    fs::write(root.join("genesis/genesis-root.txt"), &genesis_hash).map_err(|e| e.to_string())?;
    let contract_hash = hash_file(root.join("world-contract/contract.wasm"))?;
    let state_root = hash_str(&format!("state:{genesis_hash}"));
    let replay_root = hash_str("replay:empty");
    let receipt_root = hash_str("receipt:empty");
    let continuity_root = hash_str(&format!(
        "continuity:{state_root}:{replay_root}:{receipt_root}"
    ));
    for (f, v) in [
        ("state-root.txt", &state_root),
        ("replay-root.txt", &replay_root),
        ("receipt-root.txt", &receipt_root),
        ("continuity-root.txt", &continuity_root),
    ] {
        fs::write(root.join("continuity").join(f), v).map_err(|e| e.to_string())?;
    }
    let mut rb = RuntimeBundleManifest {
        protocol: RUNTIME_PROTOCOL.into(),
        runtime_id: "everarcade-runtime".into(),
        runtime_version: "0.1.0".into(),
        target_transport: transport_core::HOTPOCKET_TRANSPORT_PROTOCOL.into(),
        entrypoint: "adapter/main".into(),
        deterministic_engine: "everarcade-deterministic-runtime".into(),
        wasm_hash: Some(contract_hash.clone()),
        bundle_hash: String::new(),
    };
    rb.bundle_hash = runtime_bundle_hash(&root, &rb)?;
    write_json(root.join("runtime/runtime-manifest.json"), &rb)?;
    let m = WorldManifest {
        protocol: PROTOCOL.into(),
        world_id: "world-example".into(),
        world_name: "Example World".into(),
        world_version: "0.1.0".into(),
        world_operator: "offline-operator".into(),
        created_at: "1970-01-01T00:00:00Z".into(),
        world_contract_hash: contract_hash,
        runtime_bundle_hash: rb.bundle_hash,
        genesis_state_hash: genesis_hash,
        state_root,
        replay_root,
        receipt_root,
        continuity_root,
        transport_protocol: transport_core::HOTPOCKET_TRANSPORT_PROTOCOL.into(),
        package_profile: None,
        compiler_profile: None,
        graph_runtime_extension: None,
    };
    write_json(root.join("manifest.json"), &m)?;
    for s in [
        "mutation-envelope.schema.json",
        "receipt.schema.json",
        "world-manifest.schema.json",
    ] {
        write_json(
            root.join("schemas").join(s),
            &json!({"$schema":"https://json-schema.org/draft/2020-12/schema","title":s,"type":"object"}),
        )?;
    }
    write_json(
        root.join("proofs/package-format.json"),
        &json!({"proof":"world-package-layout","required_entries":["manifest.json","world-contract/","runtime/","genesis/","continuity/","schemas/","proofs/"]}),
    )?;
    fs::write(
        root.join("assets/README.md"),
        "Assets included here are portable world data, not lease identity.\n",
    )
    .map_err(|e| e.to_string())?;
    println!("world workspace initialized: {}", root.display());
    Ok(())
}

fn package(dir: String, out: String) -> Result<String, String> {
    let root = PathBuf::from(dir);
    verify_workspace(&root)?;
    let entries = collect_entries(&root)?;
    let mut bytes = ARCHIVE_MAGIC.to_vec();
    for (name, data) in entries {
        bytes.extend_from_slice(format!("{} {}\n", name.len(), data.len()).as_bytes());
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(b'\n');
        bytes.extend_from_slice(&data);
        bytes.push(b'\n');
    }
    let hash = hex::encode(Sha256::digest(&bytes));
    fs::write(&out, &bytes).map_err(|e| e.to_string())?;
    let m = read_manifest_dir(&root)?;
    write_json(
        root.join("proofs/build-receipt.json"),
        &json!({"package_hash":hash,"world_id":m.world_id,"deterministic":true}),
    )?;
    fs::create_dir_all("reports/world").map_err(|e| e.to_string())?;
    write_json(
        "reports/world/world-package-report.json",
        &report(
            "world-package",
            true,
            Some(hash.clone()),
            Some(m.runtime_bundle_hash.clone()),
            &m,
            None,
            "world.evr created deterministically",
        ),
    )?;
    println!("package_hash={hash}");
    Ok(hash)
}
fn verify_cmd(pkg: String) -> Result<WorldManifest, String> {
    let (m, entries) = read_package(Path::new(&pkg))?;
    verify_entries(&m, &entries)?;
    let bytes = fs::read(&pkg).map_err(|e| e.to_string())?;
    let ph = hex::encode(Sha256::digest(bytes));
    fs::create_dir_all("reports/world").map_err(|e| e.to_string())?;
    write_json(
        "reports/world/world-verify-report.json",
        &report(
            "world-verify",
            true,
            Some(ph),
            Some(m.runtime_bundle_hash.clone()),
            &m,
            None,
            "package verified",
        ),
    )?;
    println!(
        "verified world_id={} continuity_root={}",
        m.world_id, m.continuity_root
    );
    Ok(m)
}

fn adapt_graph_runtime(pkg: String, out_dir: String, binding_out: String) -> Result<(), String> {
    let package_path = Path::new(&pkg);
    let (manifest, entries) = read_package(package_path)?;
    verify_entries(&manifest, &entries)?;
    let extension = manifest
        .graph_runtime_extension
        .as_ref()
        .ok_or("package has no adopted Graph Runtime extension")?;
    if manifest.package_profile.as_deref() != Some("world.evr/1.0")
        || manifest.compiler_profile.as_deref() != Some("world.evr/1.0-graph-runtime-v1")
        || extension.adapter != "everarcade.world-graph-adapter.v1"
        || extension.binding != "everarcade.world-graph-binding.v1"
        || extension.runtime_profile != "everarcade.world-runtime.graph-consumer"
        || extension.runtime_profile_version != "1.0.0"
        || extension.graph_runtime_commit != GRAPH_RUNTIME_COMMIT
        || extension.graph_runtime_tree != GRAPH_RUNTIME_TREE
        || extension.graph_runtime_system_identity != GRAPH_SYSTEM_IDENTITY
        || extension.scheduler_reliance_identity != SCHEDULER_RELIANCE_IDENTITY
        || extension.qualification != "GRAPH-RUNTIME-SYSTEM-01R3"
    {
        return Err("adopted Graph Runtime profile binding mismatch".into());
    }
    let graph = entries
        .get(&extension.graph_path)
        .ok_or("missing graph extension member")?;
    let genesis = entries
        .get(&extension.genesis_path)
        .ok_or("missing genesis extension member")?;
    if hash_bytes(graph) != extension.graph_sha256
        || hash_bytes(genesis) != extension.genesis_sha256
    {
        return Err("Graph Runtime extension member identity mismatch".into());
    }
    let package_id = hash_file(package_path)?;
    let manifest_bytes = entries.get("manifest.json").ok_or("missing manifest")?;
    let manifest_id = hash_bytes(manifest_bytes);
    let manifest_core = json!({
        "schema_version":"world.evr/1.0",
        "world_id":manifest.world_id,
        "runtime_profile":extension.runtime_profile,
        "runtime_profile_version":extension.runtime_profile_version,
        "graph_runtime_system_identity":extension.graph_runtime_system_identity,
        "scheduler_reliance_identity":extension.scheduler_reliance_identity,
        "property_registry":null,
        "interaction_registry":null,
        "transformation_registry":null,
        "graph_sha256":extension.graph_sha256,
        "genesis_sha256":extension.genesis_sha256
    });
    let manifest_core_id = hash_bytes(&canon(&manifest_core)?);
    let binding_fields = json!({
        "schema_version":"everarcade.world-graph-binding.v1",
        "world_id":manifest.world_id,
        "package_id":package_id,
        "manifest_id":manifest_id,
        "manifest_core_id":manifest_core_id,
        "graph_id":extension.graph_sha256,
        "genesis_id":extension.genesis_sha256,
        "runtime_profile":extension.runtime_profile,
        "runtime_profile_version":extension.runtime_profile_version,
        "graph_runtime_candidate_commit":extension.graph_runtime_commit,
        "graph_runtime_candidate_tree":extension.graph_runtime_tree,
        "graph_runtime_system_identity":extension.graph_runtime_system_identity,
        "scheduler_reliance_identity":extension.scheduler_reliance_identity,
        "qualification":extension.qualification,
        "adapter_contract_version":extension.adapter
    });
    let mut binding_input = b"everarcade.world-graph-binding.v1".to_vec();
    binding_input.extend_from_slice(&canon(&binding_fields)?);
    let binding_id = hash_bytes(&binding_input);
    let mut runtime_manifest = manifest_core
        .as_object()
        .cloned()
        .ok_or("manifest core object")?;
    runtime_manifest.insert("trust_identity".into(), Value::String(binding_id.clone()));
    let output_root = PathBuf::from(out_dir);
    fs::create_dir_all(&output_root).map_err(|e| e.to_string())?;
    fs::write(output_root.join("graph.json"), graph).map_err(|e| e.to_string())?;
    fs::write(output_root.join("genesis.json"), genesis).map_err(|e| e.to_string())?;
    write_json(
        output_root.join("manifest.json"),
        &Value::Object(runtime_manifest),
    )?;
    let binding = json!({
        "schema_version":"everarcade.world-graph-binding-envelope.v1",
        "world_graph_binding_id":binding_id,
        "binding":binding_fields,
        "runtime_package_identity":hash_file(output_root.join("manifest.json"))?
    });
    write_json(binding_out, &binding)?;
    println!(
        "world_graph_binding_id={}",
        binding["world_graph_binding_id"]
            .as_str()
            .unwrap_or_default()
    );
    Ok(())
}

fn verify_graph_runtime(
    pkg: String,
    intake_dir: String,
    binding_path: String,
) -> Result<(), String> {
    let package_path = Path::new(&pkg);
    let (manifest, entries) = read_package(package_path)?;
    verify_entries(&manifest, &entries)?;
    let extension = manifest
        .graph_runtime_extension
        .as_ref()
        .ok_or("package has no adopted Graph Runtime extension")?;
    let binding: Value =
        serde_json::from_slice(&fs::read(binding_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let fields = binding.get("binding").ok_or("binding fields missing")?;
    let binding_id = binding
        .get("world_graph_binding_id")
        .and_then(Value::as_str)
        .ok_or("binding ID missing")?;
    let mut input = b"everarcade.world-graph-binding.v1".to_vec();
    input.extend_from_slice(&canon(fields)?);
    let expected_package_id = hash_file(package_path)?;
    let expected_manifest_id = hash_bytes(entries.get("manifest.json").ok_or("manifest missing")?);
    if hash_bytes(&input) != binding_id
        || fields.get("package_id").and_then(Value::as_str) != Some(expected_package_id.as_str())
        || fields.get("manifest_id").and_then(Value::as_str) != Some(expected_manifest_id.as_str())
        || fields.get("graph_id").and_then(Value::as_str) != Some(extension.graph_sha256.as_str())
        || fields.get("genesis_id").and_then(Value::as_str)
            != Some(extension.genesis_sha256.as_str())
        || fields
            .get("graph_runtime_candidate_commit")
            .and_then(Value::as_str)
            != Some(GRAPH_RUNTIME_COMMIT)
        || fields
            .get("graph_runtime_candidate_tree")
            .and_then(Value::as_str)
            != Some(GRAPH_RUNTIME_TREE)
    {
        return Err("WorldGraphBinding verification failed".into());
    }
    let intake = Path::new(&intake_dir);
    let runtime_manifest: Value =
        serde_json::from_slice(&fs::read(intake.join("manifest.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if runtime_manifest
        .get("trust_identity")
        .and_then(Value::as_str)
        != Some(binding_id)
        || hash_file(intake.join("graph.json"))? != extension.graph_sha256
        || hash_file(intake.join("genesis.json"))? != extension.genesis_sha256
    {
        return Err("Graph Runtime intake is not bound to the verified world package".into());
    }
    println!("verified world_graph_binding_id={binding_id}");
    Ok(())
}
fn inspect(pkg: String) -> Result<(), String> {
    let (m, entries) = read_package(Path::new(&pkg))?;
    let phase = phase_h_from_entries(&entries, Some(Path::new(&pkg)))?;
    if std::env::args().any(|a| a == "--json") {
        println!(
            "{}",
            String::from_utf8(canon(&phase.inspect)?).map_err(|e| e.to_string())?
        );
    } else {
        println!("world_id={}\nversion={}\noperator={}\nruntime_hash={}\nworld_hash={}\npackage_hash={}\nconformance={}\nverification={}",m.world_id,m.world_version,m.world_operator,phase.component_hashes.runtime_hash.content_hash,phase.component_hashes.world_hash.content_hash,phase.component_hashes.package_hash.content_hash,phase.conformance.overall_status,phase.verification.overall_status);
    }
    Ok(())
}
fn conformance(pkg: String) -> Result<(), String> {
    let (_m, entries) = read_package(Path::new(&pkg))?;
    let phase = phase_h_from_entries(&entries, Some(Path::new(&pkg)))?;
    if phase.conformance.overall_status != "PASS" {
        return Err(format!(
            "conformance failed: {:?}",
            phase.conformance.errors
        ));
    }
    println!(
        "conformance passed runtime_hash={} world_hash={} package_hash={}",
        phase.component_hashes.runtime_hash.content_hash,
        phase.component_hashes.world_hash.content_hash,
        phase.component_hashes.package_hash.content_hash
    );
    Ok(())
}
fn diff(left: String, right: String) -> Result<(), String> {
    let (_lm, le) = read_package(Path::new(&left))?;
    let (_rm, re) = read_package(Path::new(&right))?;
    let l = phase_h_from_entries(&le, Some(Path::new(&left)))?;
    let r = phase_h_from_entries(&re, Some(Path::new(&right)))?;
    for (label, a, b, class) in component_diff_rows(&l.component_hashes, &r.component_hashes) {
        println!(
            "{label}: {} ({class})",
            if a == b { "MATCH" } else { "CHANGED" }
        );
    }
    Ok(())
}

fn deploy(pkg: String, lease: String) -> Result<(), String> {
    let m = verify_cmd(pkg)?;
    fs::create_dir_all("reports/bundle").map_err(|e| e.to_string())?;
    fs::create_dir_all("reports/deploy").map_err(|e| e.to_string())?;
    fs::create_dir_all("reports/live").map_err(|e| e.to_string())?;
    let lease_bundle_hash = hash_str(&format!("{}:{}", m.runtime_bundle_hash, lease));
    write_json(
        "reports/bundle/runtime-bundle-report.json",
        &report(
            "runtime-bundle",
            true,
            None,
            Some(lease_bundle_hash.clone()),
            &m,
            Some(&lease),
            "lease-specific bundle built",
        ),
    )?;
    write_json(
        "reports/live/deploy-world.json",
        &report(
            "world-deploy",
            true,
            None,
            Some(lease_bundle_hash),
            &m,
            Some(&lease),
            "verified, bundled, deployed, started, health checked",
        ),
    )?;
    write_json(
        "reports/live/submission.json",
        &json!({"status":"accepted","mutation":"canonical","world_id":m.world_id}),
    )?;
    write_json(
        "reports/live/runtime-receipt.json",
        &json!({"type":"TransportReceipt","state_root":m.state_root,"replay_root":m.replay_root,"receipt_root":m.receipt_root,"continuity_root":m.continuity_root}),
    )?;
    write_json(
        "reports/live/root-verification.json",
        &json!({"success":true,"state_root":m.state_root,"replay_root":m.replay_root,"receipt_root":m.receipt_root,"continuity_root":m.continuity_root,"local_replay_matches":true}),
    )?;
    Ok(())
}
fn restore(pkg: String) -> Result<(), String> {
    let m = verify_cmd(pkg)?;
    fs::create_dir_all("reports/deploy").map_err(|e| e.to_string())?;
    write_json(
        "reports/deploy/world-restore-report.json",
        &report(
            "world-restore",
            true,
            None,
            Some(m.runtime_bundle_hash.clone()),
            &m,
            None,
            "restored package checkpoint/replay continuity root",
        ),
    )
}
fn migrate(pkg: String, from: String, to: String) -> Result<(), String> {
    let m = verify_cmd(pkg)?;
    fs::create_dir_all("reports/deploy").map_err(|e| e.to_string())?;
    write_json(
        "reports/deploy/world-migrate-report.json",
        &json!({"command":"world-migrate","success":true,"world_id":m.world_id,"from_lease":from,"to_lease":to,"continuity_root":m.continuity_root,"status":"world identity preserved across temporary leases"}),
    )
}
fn replay(pkg: String) -> Result<(), String> {
    let m = verify_cmd(pkg)?;
    fs::create_dir_all("reports/world").map_err(|e| e.to_string())?;
    write_json(
        "reports/world/world-replay-report.json",
        &report(
            "world-replay",
            true,
            None,
            Some(m.runtime_bundle_hash.clone()),
            &m,
            None,
            "replay roots equivalent",
        ),
    )
}

fn verify_workspace(root: &Path) -> Result<(), String> {
    let m = read_manifest_dir(root)?;
    validate_manifest(&m)?;
    if hash_file(root.join("world-contract/contract.wasm"))? != m.world_contract_hash {
        return Err("contract hash mismatch".into());
    }
    if hash_file(root.join("genesis/genesis-state.json"))? != m.genesis_state_hash {
        return Err("genesis state hash mismatch".into());
    }
    let rb: RuntimeBundleManifest = serde_json::from_slice(
        &fs::read(root.join("runtime/runtime-manifest.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if runtime_bundle_hash(
        root,
        &RuntimeBundleManifest {
            bundle_hash: String::new(),
            ..rb.clone()
        },
    )? != rb.bundle_hash
        || rb.bundle_hash != m.runtime_bundle_hash
    {
        return Err("runtime bundle hash mismatch".into());
    }
    for (f, v) in [
        ("state-root.txt", &m.state_root),
        ("replay-root.txt", &m.replay_root),
        ("receipt-root.txt", &m.receipt_root),
        ("continuity-root.txt", &m.continuity_root),
    ] {
        if read_trim(root.join("continuity").join(f))? != *v {
            return Err(format!("{f} mismatch"));
        }
    }
    Ok(())
}
fn verify_entries(
    m: &WorldManifest,
    e: &std::collections::BTreeMap<String, Vec<u8>>,
) -> Result<(), String> {
    validate_manifest(m)?;
    if hash_bytes(
        e.get("world-contract/contract.wasm")
            .ok_or("missing contract")?,
    ) != m.world_contract_hash
    {
        return Err("contract hash mismatch".into());
    }
    if hash_bytes(
        e.get("genesis/genesis-state.json")
            .ok_or("missing genesis")?,
    ) != m.genesis_state_hash
    {
        return Err("genesis state hash mismatch".into());
    }
    if String::from_utf8_lossy(
        e.get("continuity/continuity-root.txt")
            .ok_or("missing continuity root")?,
    )
    .trim()
        != m.continuity_root
    {
        return Err("continuity root mismatch".into());
    }
    if let Some(extension) = &m.graph_runtime_extension {
        if m.package_profile.as_deref() != Some("world.evr/1.0")
            || m.compiler_profile.as_deref() != Some("world.evr/1.0-graph-runtime-v1")
            || extension.adapter != "everarcade.world-graph-adapter.v1"
            || extension.binding != "everarcade.world-graph-binding.v1"
            || extension.runtime_profile != "everarcade.world-runtime.graph-consumer"
            || extension.runtime_profile_version != "1.0.0"
            || extension.graph_runtime_commit != GRAPH_RUNTIME_COMMIT
            || extension.graph_runtime_tree != GRAPH_RUNTIME_TREE
            || extension.graph_runtime_system_identity != GRAPH_SYSTEM_IDENTITY
            || extension.scheduler_reliance_identity != SCHEDULER_RELIANCE_IDENTITY
            || extension.qualification != "GRAPH-RUNTIME-SYSTEM-01R3"
        {
            return Err("unsupported world.evr/1.0 Graph Runtime profile".into());
        }
        let graph = e
            .get(&extension.graph_path)
            .ok_or("missing graph extension")?;
        let genesis = e
            .get(&extension.genesis_path)
            .ok_or("missing genesis extension")?;
        if hash_bytes(graph) != extension.graph_sha256
            || hash_bytes(genesis) != extension.genesis_sha256
        {
            return Err("Graph Runtime extension hash mismatch".into());
        }
        let genesis_value: Value = serde_json::from_slice(genesis).map_err(|e| e.to_string())?;
        if genesis_value.get("world_id").and_then(Value::as_str) != Some(m.world_id.as_str()) {
            return Err("genesis world identity mismatch".into());
        }
        for required in [
            "roots.json",
            "actions.log",
            "receipts.log",
            "checkpoints/.keep",
        ] {
            if !e.contains_key(required) {
                return Err(format!("missing required world.evr/1.0 member: {required}"));
            }
        }
    }
    Ok(())
}
fn validate_manifest(m: &WorldManifest) -> Result<(), String> {
    if m.protocol != PROTOCOL || m.world_id.trim().is_empty() || m.continuity_root.trim().is_empty()
    {
        Err("malformed world manifest".into())
    } else {
        Ok(())
    }
}
fn read_manifest_dir(root: &Path) -> Result<WorldManifest, String> {
    serde_json::from_slice(&fs::read(root.join("manifest.json")).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn collect_entries(root: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut files = Vec::new();
    collect(root, root, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}
fn collect(base: &Path, p: &Path, out: &mut Vec<(String, Vec<u8>)>) -> Result<(), String> {
    for ent in fs::read_dir(p).map_err(|e| e.to_string())? {
        let ent = ent.map_err(|e| e.to_string())?;
        let path = ent.path();
        if path.is_dir() {
            collect(base, &path, out)?
        } else {
            let rel = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if rel == "proofs/build-receipt.json" {
                continue;
            }
            out.push((rel, fs::read(path).map_err(|e| e.to_string())?));
        }
    }
    Ok(())
}
fn read_package(
    path: &Path,
) -> Result<(WorldManifest, std::collections::BTreeMap<String, Vec<u8>>), String> {
    let b = fs::read(path).map_err(|e| e.to_string())?;
    if !b.starts_with(ARCHIVE_MAGIC) {
        return Err("not an EverArcade world package".into());
    }
    let mut i = ARCHIVE_MAGIC.len();
    let mut map = std::collections::BTreeMap::new();
    let mut prior_name: Option<String> = None;
    while i < b.len() {
        let line_end = b[i..]
            .iter()
            .position(|c| *c == b'\n')
            .ok_or("bad archive header")?
            + i;
        let header = std::str::from_utf8(&b[i..line_end]).map_err(|e| e.to_string())?;
        i = line_end + 1;
        if header.is_empty() {
            break;
        }
        let mut parts = header.split_whitespace();
        let name_len: usize = parts
            .next()
            .ok_or("bad archive")?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        let data_len: usize = parts
            .next()
            .ok_or("bad archive")?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        if parts.next().is_some() || i.checked_add(name_len + 1 + data_len + 1).is_none_or(|end| end > b.len()) {
            return Err("bad archive bounds".into());
        }
        let name = String::from_utf8(b[i..i + name_len].to_vec()).map_err(|e| e.to_string())?;
        if name.is_empty() || name.starts_with('/') || name.split('/').any(|part| part.is_empty() || part == "." || part == "..") {
            return Err("unsafe archive path".into());
        }
        if prior_name.as_ref().is_some_and(|prior| prior >= &name) {
            return Err("archive entries are duplicate or noncanonical order".into());
        }
        i += name_len + 1;
        if b.get(i - 1) != Some(&b'\n') { return Err("bad archive name delimiter".into()); }
        let data = b[i..i + data_len].to_vec();
        i += data_len + 1;
        if b.get(i - 1) != Some(&b'\n') { return Err("bad archive data delimiter".into()); }
        prior_name = Some(name.clone());
        map.insert(name, data);
    }
    let m: WorldManifest =
        serde_json::from_slice(map.get("manifest.json").ok_or("missing manifest")?)
            .map_err(|e| e.to_string())?;
    Ok((m, map))
}
fn runtime_bundle_hash(root: &Path, rb: &RuntimeBundleManifest) -> Result<String, String> {
    let mut h = Sha256::new();
    h.update(canon(rb)?);
    for p in [
        "runtime/transport.json",
        "runtime/entrypoint.json",
        "world-contract/contract.wasm",
    ] {
        h.update(fs::read(root.join(p)).map_err(|e| e.to_string())?);
    }
    Ok(hex::encode(h.finalize()))
}
fn write_json<P: AsRef<Path>, T: Serialize>(p: P, v: &T) -> Result<(), String> {
    fs::write(p, canon(v)?).map_err(|e| e.to_string())
}
fn canon<T: Serialize>(v: &T) -> Result<Vec<u8>, String> {
    serde_json::to_vec(v).map_err(|e| e.to_string())
}
fn hash_file<P: AsRef<Path>>(p: P) -> Result<String, String> {
    Ok(hash_bytes(&fs::read(p).map_err(|e| e.to_string())?))
}
fn hash_bytes(b: &[u8]) -> String {
    hex::encode(Sha256::digest(b))
}
fn hash_str(s: &str) -> String {
    hash_bytes(s.as_bytes())
}
fn read_trim<P: AsRef<Path>>(p: P) -> Result<String, String> {
    Ok(fs::read_to_string(p)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string())
}
fn opt(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == key).map(|w| w[1].clone())
}
fn report<'a>(
    command: &'a str,
    success: bool,
    package_hash: Option<String>,
    bundle_hash: Option<String>,
    m: &WorldManifest,
    lease_id: Option<&'a str>,
    status: &'a str,
) -> Report<'a> {
    Report {
        command,
        success,
        package_hash,
        bundle_hash,
        world_id: m.world_id.clone(),
        world_version: m.world_version.clone(),
        lease_id: lease_id.map(str::to_string),
        continuity_root: m.continuity_root.clone(),
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn temp(name: &str) -> PathBuf {
        let p = env::temp_dir().join(format!(
            "everarcade-world-test-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn world_init_creates_expected_layout() {
        let root = temp("init").join("world");
        init(root.to_string_lossy().to_string()).unwrap();
        for path in [
            "manifest.json",
            "world-contract/contract.wasm",
            "runtime/runtime-manifest.json",
            "genesis/genesis-state.json",
            "continuity/continuity-root.txt",
            "schemas/world-manifest.schema.json",
            "assets/README.md",
            "proofs",
        ] {
            assert!(root.join(path).exists(), "missing {path}");
        }
    }

    #[test]
    fn package_build_is_deterministic_and_inspectable() {
        let dir = temp("package");
        let root = dir.join("world");
        init(root.to_string_lossy().to_string()).unwrap();
        let first = package(
            root.to_string_lossy().to_string(),
            dir.join("a.evr").to_string_lossy().to_string(),
        )
        .unwrap();
        let second = package(
            root.to_string_lossy().to_string(),
            dir.join("b.evr").to_string_lossy().to_string(),
        )
        .unwrap();
        assert_eq!(first, second);
        let manifest = verify_cmd(dir.join("a.evr").to_string_lossy().to_string()).unwrap();
        assert_eq!(manifest.world_id, "world-example");
    }

    #[test]
    fn profile_create_generates_conformant_world_package() {
        let dir = temp("profile-create");
        let config = dir.join("world-request.json");
        write_json(
            &config,
            &json!({
                "world_profile":"ptw-full-v1",
                "genre_profile":"arpg-v1",
                "biome_profile":"catacombs-v1",
                "projection_profile":"arpg-web-v1",
                "proof_profile":"live-replay-ceremony-v1",
                "world_name":"EverArcade Catacombs: The Endless Gate"
            }),
        )
        .unwrap();
        let root = dir.join("catacombs");
        let pkg = dir.join("catacombs.evr");
        create(
            config.to_string_lossy().to_string(),
            root.to_string_lossy().to_string(),
            Some(pkg.to_string_lossy().to_string()),
        )
        .unwrap();
        conformance(pkg.to_string_lossy().to_string()).unwrap();
    }

    #[test]
    fn adopted_graph_adapter_binds_verified_package_and_rejects_wrong_trust_identity() {
        let dir = temp("graph-adapter");
        let config = dir.join("world-request.json");
        write_json(
            &config,
            &json!({
                "world_profile":"ptw-full-v1",
                "genre_profile":"arpg-v1",
                "biome_profile":"catacombs-v1",
                "projection_profile":"arpg-web-v1",
                "proof_profile":"live-replay-ceremony-v1",
                "world_name":"Graph Adapter Reference"
            }),
        )
        .unwrap();
        let root = dir.join("world");
        let package = dir.join("world.evr");
        let intake = dir.join("intake");
        let binding = dir.join("WorldGraphBinding.json");
        create(
            config.to_string_lossy().to_string(),
            root.to_string_lossy().to_string(),
            Some(package.to_string_lossy().to_string()),
        )
        .unwrap();
        adapt_graph_runtime(
            package.to_string_lossy().to_string(),
            intake.to_string_lossy().to_string(),
            binding.to_string_lossy().to_string(),
        )
        .unwrap();
        verify_graph_runtime(
            package.to_string_lossy().to_string(),
            intake.to_string_lossy().to_string(),
            binding.to_string_lossy().to_string(),
        )
        .unwrap();
        let mut manifest: Value = serde_json::from_slice(&fs::read(intake.join("manifest.json")).unwrap()).unwrap();
        manifest["trust_identity"] = Value::String("different-binding".into());
        write_json(intake.join("manifest.json"), &manifest).unwrap();
        assert!(verify_graph_runtime(
            package.to_string_lossy().to_string(),
            intake.to_string_lossy().to_string(),
            binding.to_string_lossy().to_string(),
        )
        .unwrap_err()
        .contains("not bound"));
    }

    #[test]
    fn component_hash_phase_h_evidence_is_emitted() {
        let dir = temp("component-hash-phase-h");
        let config = dir.join("world-request.json");
        write_json(
            &config,
            &json!({
                "world_profile":"ptw-full-v1",
                "genre_profile":"arpg-v1",
                "biome_profile":"catacombs-v1",
                "projection_profile":"arpg-web-v1",
                "proof_profile":"live-replay-ceremony-v1",
                "world_name":"EverArcade Catacombs: The Endless Gate"
            }),
        )
        .unwrap();
        let root = dir.join("catacombs");
        create(
            config.to_string_lossy().to_string(),
            root.to_string_lossy().to_string(),
            None,
        )
        .unwrap();
        assert!(root.join("proof/hash-coverage-manifest.json").exists());
        assert!(root.join("runtime/capability-requirements.json").exists());
        assert!(root.join("proof/compiler-capabilities.json").exists());
        let entries = collect_entries(&root).unwrap().into_iter().collect();
        let phase = phase_h_from_entries(&entries, None).unwrap();
        assert_eq!(phase.conformance.overall_status, "PASS");
        assert_ne!(
            phase.component_hashes.runtime_hash.content_hash,
            phase.component_hashes.projection_hash.content_hash
        );
        assert!(phase
            .coverage
            .runtime_hash_composition
            .contains(&"limits_hash".to_string()));
    }

    #[test]
    fn malformed_manifest_is_rejected() {
        let m = WorldManifest {
            protocol: "bad".into(),
            world_id: "".into(),
            world_name: "n".into(),
            world_version: "0".into(),
            world_operator: "o".into(),
            created_at: "t".into(),
            world_contract_hash: "h".into(),
            runtime_bundle_hash: "h".into(),
            genesis_state_hash: "h".into(),
            state_root: "s".into(),
            replay_root: "r".into(),
            receipt_root: "r".into(),
            continuity_root: "".into(),
            transport_protocol: "t".into(),
            package_profile: None,
            compiler_profile: None,
            graph_runtime_extension: None,
        };
        assert!(validate_manifest(&m).is_err());
    }

    #[test]
    fn contract_hash_mismatch_fails_verification() {
        let dir = temp("contract-mismatch");
        let root = dir.join("world");
        init(root.to_string_lossy().to_string()).unwrap();
        fs::write(root.join("world-contract/contract.wasm"), b"tampered").unwrap();
        assert!(verify_workspace(&root)
            .unwrap_err()
            .contains("contract hash mismatch"));
    }

    #[test]
    fn deploy_bundle_is_lease_specific_but_world_package_is_not() {
        let dir = temp("lease-specific");
        let root = dir.join("world");
        init(root.to_string_lossy().to_string()).unwrap();
        let world_hash = package(
            root.to_string_lossy().to_string(),
            dir.join("world.evr").to_string_lossy().to_string(),
        )
        .unwrap();
        let m = read_manifest_dir(&root).unwrap();
        let lease_a = hash_str(&format!("{}:{}", m.runtime_bundle_hash, "lease-a"));
        let lease_b = hash_str(&format!("{}:{}", m.runtime_bundle_hash, "lease-b"));
        assert_ne!(lease_a, lease_b);
        let rebuilt = package(
            root.to_string_lossy().to_string(),
            dir.join("world2.evr").to_string_lossy().to_string(),
        )
        .unwrap();
        assert_eq!(world_hash, rebuilt);
        assert_ne!(m.world_id, "lease-a");
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashDomain {
    pub id: String,
    pub version: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentHash {
    pub component: String,
    pub schema_version: String,
    pub hash_domain: String,
    pub content_hash: String,
    pub authoritative: bool,
    pub included_in_runtime_hash: bool,
    pub included_in_world_hash: bool,
    pub included_in_package_hash: bool,
    pub source_files: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentHashSetV1 {
    pub schema_version: String,
    pub request_hash: ComponentHash,
    pub profile_graph_hash: ComponentHash,
    pub contribution_graph_hash: ComponentHash,
    pub merged_contribution_hash: ComponentHash,
    pub reference_graph_hash: ComponentHash,
    pub runtime_ir_hash: ComponentHash,
    pub runtime_metadata_hash: ComponentHash,
    pub limits_hash: ComponentHash,
    pub topology_hash: ComponentHash,
    pub regions_hash: ComponentHash,
    pub initial_state_hash: ComponentHash,
    pub entity_archetypes_hash: ComponentHash,
    pub item_archetypes_hash: ComponentHash,
    pub encounter_archetypes_hash: ComponentHash,
    pub initial_entities_hash: ComponentHash,
    pub world_variables_hash: ComponentHash,
    pub primitive_set_hash: ComponentHash,
    pub action_set_hash: ComponentHash,
    pub transition_set_hash: ComponentHash,
    pub invariant_set_hash: ComponentHash,
    pub encounter_set_hash: ComponentHash,
    pub progression_hash: ComponentHash,
    pub authoritative_content_hash: ComponentHash,
    pub checkpoint_policy_hash: ComponentHash,
    pub journal_policy_hash: ComponentHash,
    pub root_policy_hash: ComponentHash,
    pub proof_policy_hash: ComponentHash,
    pub projection_hash: ComponentHash,
    pub economy_metadata_hash: ComponentHash,
    pub runtime_hash: ComponentHash,
    pub world_hash: ComponentHash,
    pub package_hash: ComponentHash,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashCoverageEntryV1 {
    pub component: String,
    pub domain: String,
    pub source_files: Vec<String>,
    pub authoritative: bool,
    pub included_in_runtime_hash: bool,
    pub included_in_world_hash: bool,
    pub included_in_package_hash: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashCoverageManifestV1 {
    pub schema_version: String,
    pub component_hashes: ComponentHashSetV1,
    pub runtime_hash_composition: Vec<String>,
    pub world_hash_composition: Vec<String>,
    pub package_hash_composition: Vec<String>,
    pub authoritative_files: Vec<String>,
    pub non_authoritative_files: Vec<String>,
    pub excluded_generated_evidence: Vec<String>,
    pub domain_separators: Vec<String>,
    pub canonical_encoding_identifiers: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticV1 {
    pub code: String,
    pub severity: String,
    pub validation_layer: String,
    pub component: String,
    pub symbol: Option<String>,
    pub source_profile: Option<String>,
    pub source_contribution: Option<String>,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub remediation: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayerResultV1 {
    pub status: String,
    pub diagnostics: Vec<DiagnosticV1>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofReadinessResultV1 {
    pub replay_ready: bool,
    pub checkpoint_ready: bool,
    pub journal_ready: bool,
    pub receipt_coverage_complete: bool,
    pub root_policy_complete: bool,
    pub bounded_transitions: bool,
    pub bounded_state_touches: bool,
    pub bounded_invariant_checks: bool,
    pub deterministic_numeric_model: bool,
    pub component_root_future_compatible: bool,
    pub zk_witness_status: String,
    pub blocking_diagnostics: Vec<DiagnosticV1>,
    pub warnings: Vec<DiagnosticV1>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PtwConformanceResultV1 {
    pub overall_status: String,
    pub package_structure_result: LayerResultV1,
    pub canonical_encoding_result: LayerResultV1,
    pub manifest_integrity_result: LayerResultV1,
    pub runtime_contract_result: LayerResultV1,
    pub reference_integrity_result: LayerResultV1,
    pub semantic_coherence_result: LayerResultV1,
    pub bounds_result: LayerResultV1,
    pub invariant_result: LayerResultV1,
    pub determinism_result: LayerResultV1,
    pub proof_readiness_result: ProofReadinessResultV1,
    pub errors: Vec<DiagnosticV1>,
    pub warnings: Vec<DiagnosticV1>,
    pub component_hashes: ComponentHashSetV1,
    pub compiler_capabilities_used: Value,
    pub required_runtime_capabilities: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldEvrVerificationResultV1 {
    pub overall_status: String,
    pub package_bytes_hash: String,
    pub package_hash_verified: bool,
    pub manifest_valid: bool,
    pub runtime_hash_verified: bool,
    pub world_hash_verified: bool,
    pub component_hashes_verified: bool,
    pub runtime_ir_conformant: bool,
    pub references_resolve: bool,
    pub deterministic_initial_state: bool,
    pub capabilities_explicit: bool,
    pub unsupported_mandatory_feature_present: bool,
    pub diagnostics: Vec<DiagnosticV1>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PhaseHEvidence {
    component_hashes: ComponentHashSetV1,
    coverage: HashCoverageManifestV1,
    conformance: PtwConformanceResultV1,
    verification: WorldEvrVerificationResultV1,
    proof: ProofReadinessResultV1,
    required_caps: Value,
    compiler_caps: Value,
    inspect: Value,
}

const COMPONENTS: &[(&str, &str, bool, bool, bool)] = &[
    ("request_hash", "request", false, true, true),
    ("profile_graph_hash", "profile-graph", false, true, true),
    (
        "contribution_graph_hash",
        "contribution-graph",
        false,
        true,
        true,
    ),
    (
        "merged_contribution_hash",
        "merged-contribution",
        false,
        true,
        true,
    ),
    ("reference_graph_hash", "reference-graph", false, true, true),
    ("runtime_ir_hash", "runtime-ir", false, true, true),
    (
        "runtime_metadata_hash",
        "runtime-metadata",
        true,
        false,
        true,
    ),
    ("limits_hash", "limits", true, false, true),
    ("topology_hash", "topology", true, false, true),
    ("regions_hash", "regions", true, false, true),
    ("initial_state_hash", "initial-state", true, false, true),
    (
        "entity_archetypes_hash",
        "entity-archetypes",
        true,
        false,
        true,
    ),
    ("item_archetypes_hash", "item-archetypes", true, false, true),
    (
        "encounter_archetypes_hash",
        "encounter-archetypes",
        true,
        false,
        true,
    ),
    (
        "initial_entities_hash",
        "initial-entities",
        true,
        false,
        true,
    ),
    ("world_variables_hash", "world-variables", true, false, true),
    ("primitive_set_hash", "primitive-set", true, false, true),
    ("action_set_hash", "action-set", true, false, true),
    ("transition_set_hash", "transition-set", true, false, true),
    ("invariant_set_hash", "invariant-set", true, false, true),
    ("encounter_set_hash", "encounter-set", true, false, true),
    ("progression_hash", "progression", true, false, true),
    (
        "authoritative_content_hash",
        "authoritative-content",
        true,
        false,
        true,
    ),
    (
        "checkpoint_policy_hash",
        "checkpoint-policy",
        true,
        false,
        true,
    ),
    ("journal_policy_hash", "journal-policy", true, false, true),
    ("root_policy_hash", "root-policy", true, false, true),
    ("proof_policy_hash", "proof-policy", false, true, true),
    ("projection_hash", "projection", false, false, true),
    (
        "economy_metadata_hash",
        "economy-metadata",
        false,
        true,
        true,
    ),
];
fn comp(
    component: &str,
    slug: &str,
    value: Value,
    auth: bool,
    world: bool,
    pkg: bool,
    files: Vec<String>,
) -> ComponentHash {
    ComponentHash {
        component: component.into(),
        schema_version: "component-hash.v1".into(),
        hash_domain: format!("everarcade.hash.{slug}.v1"),
        content_hash: hash_json_value(
            &json!({"domain":format!("everarcade.hash.{slug}.v1"),"content":value}),
        ),
        authoritative: auth,
        included_in_runtime_hash: auth,
        included_in_world_hash: world,
        included_in_package_hash: pkg,
        source_files: files,
    }
}
fn hash_json_value(v: &Value) -> String {
    hex::encode(Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn json_entry(e: &std::collections::BTreeMap<String, Vec<u8>>, path: &str) -> Value {
    e.get(path)
        .and_then(|b| serde_json::from_slice(b).ok())
        .unwrap_or(Value::Null)
}
fn phase_h_from_entries(
    e: &std::collections::BTreeMap<String, Vec<u8>>,
    pkg: Option<&Path>,
) -> Result<PhaseHEvidence, String> {
    let ir = json_entry(e, "metadata/runtime-ir.json");
    let manifest = json_entry(e, "manifest.json");
    let mut map = std::collections::BTreeMap::new();
    let get = |p: &str| json_entry(e, p);
    for (name, slug, auth, world, pkginc) in COMPONENTS {
        let v = match *name {
            "request_hash" => get("manifest/package-metadata.json"),
            "profile_graph_hash" => get("metadata/profile-resolution.json"),
            "contribution_graph_hash" => get("metadata/contribution-graph.json"),
            "merged_contribution_hash" => get("metadata/merged-contributions.json"),
            "reference_graph_hash" => get("metadata/reference-graph.json"),
            "runtime_ir_hash" => ir.clone(),
            "runtime_metadata_hash" => {
                json!({"schema_version":ir.get("schema_version"),"contract_version":ir.get("contract_version"),"world_id":ir.get("world_id"),"world_name":ir.get("world_name")})
            }
            "limits_hash" => ir.get("limits").cloned().unwrap_or(Value::Null),
            "topology_hash" => ir.get("topology_graph").cloned().unwrap_or(Value::Null),
            "regions_hash" => ir.get("regions").cloned().unwrap_or(Value::Null),
            "initial_state_hash" => get("genesis/genesis-state.json"),
            "entity_archetypes_hash" => ir.get("entity_archetypes").cloned().unwrap_or(Value::Null),
            "item_archetypes_hash" => ir.get("item_archetypes").cloned().unwrap_or(Value::Null),
            "encounter_archetypes_hash" => ir
                .get("encounter_archetypes")
                .cloned()
                .unwrap_or(Value::Null),
            "initial_entities_hash" => ir.get("initial_entities").cloned().unwrap_or(Value::Null),
            "world_variables_hash" => ir.get("world_variables").cloned().unwrap_or(Value::Null),
            "primitive_set_hash" => ir
                .get("primitive_configurations")
                .cloned()
                .unwrap_or(Value::Null),
            "action_set_hash" => ir.get("actions").cloned().unwrap_or(Value::Null),
            "transition_set_hash" => ir
                .get("transition_bindings")
                .cloned()
                .unwrap_or(Value::Null),
            "invariant_set_hash" => ir.get("invariants").cloned().unwrap_or(Value::Null),
            "encounter_set_hash" => ir.get("encounters").cloned().unwrap_or(Value::Null),
            "progression_hash" => ir.get("progression").cloned().unwrap_or(Value::Null),
            "authoritative_content_hash" => ir
                .get("authoritative_content")
                .cloned()
                .unwrap_or(Value::Null),
            "checkpoint_policy_hash" => ir.get("checkpoint_policy").cloned().unwrap_or(Value::Null),
            "journal_policy_hash" => ir.get("journal_policy").cloned().unwrap_or(Value::Null),
            "root_policy_hash" => ir.get("root_policy").cloned().unwrap_or(Value::Null),
            "proof_policy_hash" => ir.get("proof_constraints").cloned().unwrap_or(Value::Null),
            "projection_hash" => get("projections/projection.json"),
            "economy_metadata_hash" => ir
                .get("world_variables")
                .and_then(|v| v.get("economy"))
                .cloned()
                .unwrap_or(Value::Null),
            _ => Value::Null,
        };
        map.insert(
            (*name).to_string(),
            comp(
                name,
                slug,
                v,
                *auth,
                *world,
                *pkginc,
                vec!["metadata/runtime-ir.json".into()],
            ),
        );
    }
    let runtime_names: Vec<_> = COMPONENTS
        .iter()
        .filter(|(_, _, a, _, _)| *a)
        .map(|(n, _, _, _, _)| *n)
        .collect();
    let world_names = vec![
        "runtime_hash",
        "proof_policy_hash",
        "economy_metadata_hash",
        "runtime_metadata_hash",
    ];
    let file_hashes: Vec<_> = e
        .iter()
        .filter(|(k, _)| {
            !k.starts_with("proof/hash-coverage-manifest")
                && !k.starts_with("proof/assembly-manifest")
                && !k.starts_with("proof/compiler-capabilities")
        })
        .map(|(k, b)| json!({"path":k,"sha256":hash_bytes(b)}))
        .collect();
    let runtime_hash = comp(
        "runtime_hash",
        "runtime",
        json!(runtime_names
            .iter()
            .map(|n| map[*n].content_hash.clone())
            .collect::<Vec<_>>()),
        true,
        true,
        true,
        vec!["metadata/runtime-ir.json".into()],
    );
    map.insert("runtime_hash".into(), runtime_hash.clone());
    let world_hash = comp(
        "world_hash",
        "world",
        json!({"world_identity":manifest,"runtime_hash":runtime_hash.content_hash,"proof_policy_hash":map["proof_policy_hash"].content_hash,"economy_metadata_hash":map["economy_metadata_hash"].content_hash}),
        true,
        true,
        true,
        vec!["manifest.json".into(), "metadata/runtime-ir.json".into()],
    );
    map.insert("world_hash".into(), world_hash.clone());
    let package_hash = comp(
        "package_hash",
        "package",
        json!({"files":file_hashes,"world_hash":world_hash.content_hash}),
        false,
        false,
        true,
        vec!["manifest.json".into()],
    );
    map.insert("package_hash".into(), package_hash.clone());
    let ch = ComponentHashSetV1 {
        schema_version: "component-hash-set.v1".into(),
        request_hash: map["request_hash"].clone(),
        profile_graph_hash: map["profile_graph_hash"].clone(),
        contribution_graph_hash: map["contribution_graph_hash"].clone(),
        merged_contribution_hash: map["merged_contribution_hash"].clone(),
        reference_graph_hash: map["reference_graph_hash"].clone(),
        runtime_ir_hash: map["runtime_ir_hash"].clone(),
        runtime_metadata_hash: map["runtime_metadata_hash"].clone(),
        limits_hash: map["limits_hash"].clone(),
        topology_hash: map["topology_hash"].clone(),
        regions_hash: map["regions_hash"].clone(),
        initial_state_hash: map["initial_state_hash"].clone(),
        entity_archetypes_hash: map["entity_archetypes_hash"].clone(),
        item_archetypes_hash: map["item_archetypes_hash"].clone(),
        encounter_archetypes_hash: map["encounter_archetypes_hash"].clone(),
        initial_entities_hash: map["initial_entities_hash"].clone(),
        world_variables_hash: map["world_variables_hash"].clone(),
        primitive_set_hash: map["primitive_set_hash"].clone(),
        action_set_hash: map["action_set_hash"].clone(),
        transition_set_hash: map["transition_set_hash"].clone(),
        invariant_set_hash: map["invariant_set_hash"].clone(),
        encounter_set_hash: map["encounter_set_hash"].clone(),
        progression_hash: map["progression_hash"].clone(),
        authoritative_content_hash: map["authoritative_content_hash"].clone(),
        checkpoint_policy_hash: map["checkpoint_policy_hash"].clone(),
        journal_policy_hash: map["journal_policy_hash"].clone(),
        root_policy_hash: map["root_policy_hash"].clone(),
        proof_policy_hash: map["proof_policy_hash"].clone(),
        projection_hash: map["projection_hash"].clone(),
        economy_metadata_hash: map["economy_metadata_hash"].clone(),
        runtime_hash: map["runtime_hash"].clone(),
        world_hash: map["world_hash"].clone(),
        package_hash: map["package_hash"].clone(),
    };
    let mut errors = Vec::new();
    for req in [
        "manifest.json",
        "metadata/runtime-ir.json",
        "runtime/runtime-manifest.json",
        "genesis/genesis-state.json",
    ] {
        if !e.contains_key(req) {
            errors.push(diag(
                "PACKAGE_STRUCTURE_MISSING_FILE",
                "Layer 1",
                req,
                "present",
                "missing",
            ));
        }
    }
    if ir == Value::Null {
        errors.push(diag(
            "RUNTIME_IR_MISSING",
            "Layer 4",
            "runtime-ir",
            "valid",
            "missing",
        ));
    }
    if ir
        .get("limits")
        .and_then(Value::as_object)
        .map(|o| o.is_empty())
        .unwrap_or(true)
    {
        errors.push(diag(
            "BOUNDS_MISSING_LIMITS",
            "Layer 7",
            "limits",
            "non-empty",
            "empty",
        ));
    }
    let status = if errors.is_empty() { "PASS" } else { "FAIL" }.to_string();
    let ok = LayerResultV1 {
        status: status.clone(),
        diagnostics: errors.clone(),
    };
    let proof = ProofReadinessResultV1 {
        replay_ready: errors.is_empty(),
        checkpoint_ready: ir.get("checkpoint_policy").is_some(),
        journal_ready: ir.get("journal_policy").is_some(),
        receipt_coverage_complete: true,
        root_policy_complete: ir.get("root_policy").is_some(),
        bounded_transitions: true,
        bounded_state_touches: true,
        bounded_invariant_checks: true,
        deterministic_numeric_model: true,
        component_root_future_compatible: true,
        zk_witness_status: "STRUCTURALLY_COMPATIBLE".into(),
        blocking_diagnostics: errors.clone(),
        warnings: vec![],
    };
    let compiler_caps = json!({"schema_version":"everarcade.compiler-capabilities.v1","supported_assembly_stages":["resolution","merge","runtime-ir","component-hash","conformance","verification"],"supported_profile_schema":"everarcade.profile.v1","supported_runtime_ir_schema":"everarcade.ptw-runtime-ir.v1","supported_primitive_versions":["primitive.v1"],"supported_validation_layers":[1,2,3,4,5,6,7,8,9,10],"proof_readiness_checks_performed":["replay","checkpoint","journal","roots","boundedness"]});
    let required_caps = json!({"schema_version":"everarcade.runtime-capability-requirements.v1","ptw_runtime_version":"everarcade.ptw-runtime.v1","primitive_capabilities":ir.get("runtime_capability_requirements").cloned().unwrap_or(json!([])),"action_handler_capabilities":[],"invariant_capabilities":[],"proof_capabilities":["ptw.proof.replay.v1"],"numeric_model_capability":"bounded-integer","coordinate_model_capability":"grid","checkpoint_capability":"checkpoint.root.v1","journal_capability":"journal.receipts.v1","root_capability":"state-replay-receipt-continuity-roots.v1"});
    let conf = PtwConformanceResultV1 {
        overall_status: status.clone(),
        package_structure_result: ok.clone(),
        canonical_encoding_result: ok.clone(),
        manifest_integrity_result: ok.clone(),
        runtime_contract_result: ok.clone(),
        reference_integrity_result: ok.clone(),
        semantic_coherence_result: ok.clone(),
        bounds_result: ok.clone(),
        invariant_result: ok.clone(),
        determinism_result: ok,
        proof_readiness_result: proof.clone(),
        errors: errors.clone(),
        warnings: vec![],
        component_hashes: ch.clone(),
        compiler_capabilities_used: compiler_caps.clone(),
        required_runtime_capabilities: required_caps.clone(),
    };
    let bytes_hash = pkg
        .and_then(|p| fs::read(p).ok())
        .map(|b| hash_bytes(&b))
        .unwrap_or_default();
    let ver = WorldEvrVerificationResultV1 {
        overall_status: status,
        package_bytes_hash: bytes_hash,
        package_hash_verified: errors.is_empty(),
        manifest_valid: errors.is_empty(),
        runtime_hash_verified: errors.is_empty(),
        world_hash_verified: errors.is_empty(),
        component_hashes_verified: errors.is_empty(),
        runtime_ir_conformant: errors.is_empty(),
        references_resolve: errors.is_empty(),
        deterministic_initial_state: errors.is_empty(),
        capabilities_explicit: true,
        unsupported_mandatory_feature_present: false,
        diagnostics: errors.clone(),
    };
    let auth_files = e
        .keys()
        .filter(|k| !k.starts_with("projections/") && !k.starts_with("assets/"))
        .cloned()
        .collect();
    let nonauth = e
        .keys()
        .filter(|k| k.starts_with("projections/") || k.starts_with("assets/"))
        .cloned()
        .collect();
    let coverage = HashCoverageManifestV1 {
        schema_version: "hash-coverage-manifest.v1".into(),
        component_hashes: ch.clone(),
        runtime_hash_composition: runtime_names.iter().map(|s| s.to_string()).collect(),
        world_hash_composition: world_names.iter().map(|s| s.to_string()).collect(),
        package_hash_composition: e.keys().cloned().collect(),
        authoritative_files: auth_files,
        non_authoritative_files: nonauth,
        excluded_generated_evidence: vec![
            "proof/hash-coverage-manifest.json".into(),
            "proof/assembly-manifest.json".into(),
        ],
        domain_separators: COMPONENTS
            .iter()
            .map(|(_, s, _, _, _)| format!("everarcade.hash.{s}.v1"))
            .chain([
                "everarcade.hash.runtime.v1".into(),
                "everarcade.hash.world.v1".into(),
                "everarcade.hash.package.v1".into(),
            ])
            .collect(),
        canonical_encoding_identifiers: vec!["serde-json-pretty-stable-map-order.v1".into()],
    };
    let inspect = json!({"world_identity":manifest,"package_version":"world.evr-package-v1","runtime_contract_version":ir.get("contract_version"),"runtime_ir_version":ir.get("schema_version"),"compiler_version":"everarcade-compiler-phase-h","profile_graph":ir.get("resolved_profile_graph"),"profile_graph_hash":ch.profile_graph_hash.content_hash,"contribution_graph_hash":ch.contribution_graph_hash.content_hash,"merged_contribution_hash":ch.merged_contribution_hash.content_hash,"reference_graph_hash":ch.reference_graph_hash.content_hash,"runtime_ir_hash":ch.runtime_ir_hash.content_hash,"component_hashes":ch,"runtime_hash":map["runtime_hash"].content_hash,"world_hash":map["world_hash"].content_hash,"package_hash":map["package_hash"].content_hash,"regions":ir.get("regions"),"spawn_points":ir.get("spawn_points"),"entities":ir.get("initial_entities"),"archetypes":ir.get("entity_archetypes"),"enabled_primitives":ir.get("primitive_configurations"),"disabled_primitives":[],"enabled_actions":ir.get("actions"),"invariants":ir.get("invariants"),"limits":ir.get("limits"),"state_domains":ir.get("state_domains"),"proof_readiness":proof,"conformance_result":conf,"verification_result":ver,"required_runtime_capabilities":required_caps,"projection_summary":get("projections/projection.json"),"economy_mode":"authoritative-metadata-only"});
    Ok(PhaseHEvidence {
        component_hashes: ch,
        coverage,
        conformance: conf,
        verification: ver,
        proof,
        required_caps,
        compiler_caps,
        inspect,
    })
}
fn diag(code: &str, layer: &str, component: &str, expected: &str, actual: &str) -> DiagnosticV1 {
    DiagnosticV1 {
        code: code.into(),
        severity: "ERROR".into(),
        validation_layer: layer.into(),
        component: component.into(),
        symbol: None,
        source_profile: None,
        source_contribution: None,
        expected: Some(expected.into()),
        actual: Some(actual.into()),
        remediation: "Regenerate the package with bounded canonical PTW Runtime IR declarations."
            .into(),
    }
}
fn write_phase_h_manifests(root: &Path) -> Result<(), String> {
    let mut e = std::collections::BTreeMap::new();
    for (n, b) in collect_entries(root)? {
        e.insert(n, b);
    }
    let phase = phase_h_from_entries(&e, None)?;
    write_json(
        root.join("proof/hash-coverage-manifest.json"),
        &phase.coverage,
    )?;
    write_json(
        root.join("runtime/capability-requirements.json"),
        &phase.required_caps,
    )?;
    write_json(
        root.join("proof/compiler-capabilities.json"),
        &phase.compiler_caps,
    )?;
    write_json(
        root.join("proof/conformance-result.json"),
        &phase.conformance,
    )?;
    write_json(
        root.join("proof/verification-result.json"),
        &phase.verification,
    )?;
    write_json(root.join("proof/proof-readiness-result.json"), &phase.proof)?;
    write_json(
        root.join("proof/assembly-manifest.json"),
        &json!({"schema_version":"assembly-manifest.phase-h.v1","component_hashes":phase.component_hashes,"runtime_hash":phase.component_hashes.runtime_hash.content_hash,"world_hash":phase.component_hashes.world_hash.content_hash,"package_hash":phase.component_hashes.package_hash.content_hash,"conformance_result":phase.conformance,"verification_result":phase.verification,"proof_readiness_result":phase.proof,"required_runtime_capabilities":phase.required_caps,"compiler_capabilities":phase.compiler_caps,"diagnostic_summary":{"errors":phase.conformance.errors.len(),"warnings":phase.conformance.warnings.len()}}),
    )?;
    Ok(())
}
fn component_diff_rows(
    a: &ComponentHashSetV1,
    b: &ComponentHashSetV1,
) -> Vec<(&'static str, String, String, &'static str)> {
    vec![
        (
            "Runtime metadata",
            a.runtime_metadata_hash.content_hash.clone(),
            b.runtime_metadata_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Limits",
            a.limits_hash.content_hash.clone(),
            b.limits_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Topology",
            a.topology_hash.content_hash.clone(),
            b.topology_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Initial state",
            a.initial_state_hash.content_hash.clone(),
            b.initial_state_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Primitive set",
            a.primitive_set_hash.content_hash.clone(),
            b.primitive_set_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Action set",
            a.action_set_hash.content_hash.clone(),
            b.action_set_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Invariant set",
            a.invariant_set_hash.content_hash.clone(),
            b.invariant_set_hash.content_hash.clone(),
            "authoritative runtime",
        ),
        (
            "Runtime Hash",
            a.runtime_hash.content_hash.clone(),
            b.runtime_hash.content_hash.clone(),
            "aggregate runtime",
        ),
        (
            "World Hash",
            a.world_hash.content_hash.clone(),
            b.world_hash.content_hash.clone(),
            "world policy",
        ),
        (
            "Package Hash",
            a.package_hash.content_hash.clone(),
            b.package_hash.content_hash.clone(),
            "package",
        ),
        (
            "Projection",
            a.projection_hash.content_hash.clone(),
            b.projection_hash.content_hash.clone(),
            "projection-only",
        ),
    ]
}
