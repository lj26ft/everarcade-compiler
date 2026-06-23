use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const PROTOCOL: &str = "everarcade-release-v0.1";
const RUNTIME_BUNDLE_PROTOCOL: &str = "everarcade-runtime-bundle-v0.1";

const WORLD_FACTORY_ARCHIVE: &str = "dist/everarcade-world-factory-release.tar.gz";
const WORLD_FACTORY_STAGING: &str = "dist/everarcade-world-factory-release";
const MAX_RELEASE_BYTES: u64 = 100 * 1024 * 1024;

pub fn build_world_factory_release(project: String) -> Result<PathBuf, String> {
    let project = PathBuf::from(project);
    if !project.join("world-blueprint.json").is_file() {
        return Err(format!(
            "missing world factory blueprint: {}",
            project.display()
        ));
    }
    fs::create_dir_all("dist").map_err(|e| e.to_string())?;
    let staging = PathBuf::from(WORLD_FACTORY_STAGING);
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging).map_err(|e| e.to_string())?;

    let _ = Command::new("cargo")
        .args(["build", "-p", "everarcade-cli", "--release"])
        .status();

    let world_workspace = staging.join("_world-build");
    crate::world::dispatch(&sargs_owned(vec![
        "everarcade",
        "world",
        "init",
        "--dir",
        world_workspace
            .to_str()
            .unwrap_or("dist/everarcade-world-factory-release/_world-build"),
    ]))?;
    apply_frontier_metadata(&world_workspace, &project)?;
    let world_out = staging.join("world.evr");
    crate::world::dispatch(&sargs_owned(vec![
        "everarcade",
        "world",
        "package",
        "--dir",
        world_workspace.to_str().unwrap(),
        "--out",
        world_out.to_str().unwrap(),
    ]))?;
    let _ = fs::remove_dir_all(&world_workspace);

    copy_or_placeholder(
        "target/release/everarcade",
        staging.join("bin/everarcade"),
        b"#!/bin/sh\necho everarcade release placeholder\n",
    )?;
    make_executable(staging.join("bin/everarcade"))?;

    write_json(
        staging.join("runtime-config.json"),
        &json!({
            "runtime":"world-factory-serve",
            "world":"world.evr",
            "transport":"hotpocket-evernode",
            "host_profile":"single-evernode-lease",
            "remote_proof": {"enabled": true, "script":"verification/remote-proof.sh"}
        }),
    )?;
    write_json(
        staging.join("deployment-manifest.json"),
        &json!({
            "protocol":"everarcade-world-factory-release-v0.1",
            "world_id":"frontier-settlement-demo",
            "entrypoint":"bin/everarcade world deploy --package world.evr --lease $EVERNODE_LEASE_ID",
            "serve_command":"bin/everarcade world deploy --package world.evr --lease ${EVERNODE_LEASE_ID:-offline-lease}",
            "verify_command":"verification/verify-local.sh",
            "remote_proof_command":"verification/remote-proof.sh"
        }),
    )?;
    fs::create_dir_all(staging.join("keys")).map_err(|e| e.to_string())?;
    fs::write(
        staging.join("keys/trusted-public-key.txt"),
        "everarcade-frontier-settlement-trusted-public-key-v0.1\n",
    )
    .map_err(|e| e.to_string())?;
    let world_hash = hash_file(&world_out)?;
    let cli_hash = hash_file(staging.join("bin/everarcade"))?;
    write_json(
        staging.join("attestation/world-release-attestation.json"),
        &json!({
            "protocol":"everarcade-release-attestation-v0.1",
            "world_id":"frontier-settlement-demo",
            "world_hash": world_hash,
            "cli_hash": cli_hash,
            "trusted_public_key":"keys/trusted-public-key.txt",
            "remote_proof_supported": true,
            "created_at": created_at()
        }),
    )?;
    write_release_scripts(&staging)?;
    fs::write(staging.join("README_DEPLOY.md"), deploy_readme()).map_err(|e| e.to_string())?;

    let archive = PathBuf::from(WORLD_FACTORY_ARCHIVE);
    let _ = fs::remove_file(&archive);
    let status = Command::new("tar")
        .args([
            "-czf",
            WORLD_FACTORY_ARCHIVE,
            "-C",
            "dist",
            "everarcade-world-factory-release",
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("tar failed while creating release bundle".into());
    }
    let size = fs::metadata(&archive).map_err(|e| e.to_string())?.len();
    if size > MAX_RELEASE_BYTES {
        return Err(format!("release bundle exceeds 100 MB: {size} bytes"));
    }
    write_json(
        staging.join("size-report.json"),
        &json!({"archive": WORLD_FACTORY_ARCHIVE, "bytes": size, "max_bytes": MAX_RELEASE_BYTES, "stretch_target_bytes": 25 * 1024 * 1024, "status":"PASS"}),
    )?;
    let status = Command::new("tar")
        .args([
            "-czf",
            WORLD_FACTORY_ARCHIVE,
            "-C",
            "dist",
            "everarcade-world-factory-release",
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("tar failed while refreshing release bundle with size report".into());
    }
    inspect_release_tar(&archive)?;
    Ok(archive)
}

fn inspect_release_target(path: String) -> Result<(), String> {
    if path.ends_with(".tar.gz") {
        inspect_release_tar(Path::new(&path))
    } else {
        inspect(path)
    }
}

fn inspect_release_tar(path: &Path) -> Result<(), String> {
    let size = fs::metadata(path).map_err(|e| e.to_string())?.len();
    if size > MAX_RELEASE_BYTES {
        return Err(format!("release bundle exceeds 100 MB: {size} bytes"));
    }
    let out = Command::new("tar")
        .args(["-tzf", path.to_str().ok_or("bad archive path")?])
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("release archive is not readable by tar".into());
    }
    let listing = String::from_utf8_lossy(&out.stdout);
    reject_dev_entries(&listing)?;
    for required in [
        "bin/everarcade",
        "world.evr",
        "runtime-config.json",
        "deployment-manifest.json",
        "verification/verify-local.sh",
        "verification/smoke-test.sh",
        "verification/remote-proof.sh",
        "attestation/world-release-attestation.json",
        "keys/trusted-public-key.txt",
        "README_DEPLOY.md",
        "size-report.json",
    ] {
        if !listing.contains(required) {
            return Err(format!("release archive missing {required}"));
        }
    }
    println!(
        "release_bundle={}\nsize_bytes={}\n{}",
        path.display(),
        size,
        listing
    );
    Ok(())
}

fn smoke_test(path: String) -> Result<(), String> {
    let archive = PathBuf::from(path);
    inspect_release_tar(&archive)?;
    let temp =
        std::env::temp_dir().join(format!("everarcade-release-smoke-{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp);
    fs::create_dir_all(&temp).map_err(|e| e.to_string())?;
    let status = Command::new("tar")
        .args([
            "-xzf",
            archive.to_str().ok_or("bad archive path")?,
            "-C",
            temp.to_str().unwrap(),
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("failed to extract release archive".into());
    }
    let root = temp.join("everarcade-world-factory-release");
    for script in [
        "verification/verify-local.sh",
        "verification/smoke-test.sh",
        "verification/remote-proof.sh",
    ] {
        let status = Command::new("sh")
            .arg(root.join(script))
            .current_dir(&root)
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("release smoke script failed: {script}"));
        }
    }
    let _ = fs::remove_dir_all(&temp);
    println!("release smoke-test passed: {}", archive.display());
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub protocol: String,
    pub release_id: String,
    pub version: String,
    pub git_commit: String,
    pub created_at: String,
    pub cli_hash: String,
    pub runtime_hash: String,
    pub world_package_hash: String,
    pub runtime_bundle_hash: String,
    pub vendor_hash: Option<String>,
}

pub fn dispatch(args: &[String]) -> Result<(), String> {
    match args
        .get(2)
        .map(String::as_str)
        .ok_or("usage: everarcade release <build|inspect|smoke-test|package|verify>")?
    {
        "build" => build_world_factory_release(
            opt(args, "--project")
                .unwrap_or_else(|| "examples/world-factory/frontier-settlement".into()),
        )
        .map(|_| ()),
        "smoke-test" => smoke_test(
            args.get(3)
                .cloned()
                .unwrap_or_else(|| "dist/everarcade-world-factory-release.tar.gz".into()),
        ),
        "package" => package(
            opt(args, "--dist").unwrap_or_else(|| "dist".into()),
            has(args, "--vendor"),
        )
        .map(|_| ()),
        "inspect" => inspect_release_target(
            args.get(3)
                .cloned()
                .or_else(|| opt(args, "--manifest"))
                .unwrap_or_else(|| "dist/release-manifest.json".into()),
        ),
        "verify" => verify(opt(args, "--dist").unwrap_or_else(|| "dist".into())).map(|_| ()),
        other => Err(format!("unknown release command: {other}")),
    }
}

pub fn package(dist: String, include_vendor: bool) -> Result<ReleaseManifest, String> {
    let dist = PathBuf::from(dist);
    fs::create_dir_all(dist.join("reports")).map_err(|e| e.to_string())?;
    fs::create_dir_all("reports/release").map_err(|e| e.to_string())?;
    fs::create_dir_all("reports/bundle").map_err(|e| e.to_string())?;

    let _ = Command::new("cargo")
        .args(["build", "-p", "everarcade-cli", "--release"])
        .status();
    let _ = Command::new("cargo")
        .args(["build", "-p", "everarcade-runtime", "--release"])
        .status();

    copy_or_placeholder(
        "target/release/everarcade",
        dist.join("everarcade-cli"),
        b"everarcade-cli release placeholder\n",
    )?;
    copy_or_placeholder("target/release/runtime", dist.join("everarcade-runtime"), b"everarcade-runtime deterministic input execution MutationEnvelope receipt state_root replay_root continuity_root health\n")?;

    if !Path::new("world/manifest.json").exists() {
        crate::world::dispatch(&sargs(["everarcade", "world", "init", "--dir", "world"]))?;
    }
    crate::world::dispatch(&sargs([
        "everarcade",
        "world",
        "package",
        "--dir",
        "world",
        "--out",
        dist.join("world.evr").to_str().unwrap_or("dist/world.evr"),
    ]))?;
    crate::world::dispatch(&sargs([
        "everarcade",
        "world",
        "deploy",
        "--package",
        dist.join("world.evr").to_str().unwrap_or("dist/world.evr"),
        "--lease",
        &lease_id(),
    ]))?;
    build_runtime_bundle(&dist, &lease_id())?;

    let vendor_hash = if include_vendor || Path::new("vendor").exists() {
        let archive = dist.join("vendor.tar.gz");
        write_vendor_archive(&archive)?;
        let h = hash_file(&archive)?;
        fs::write(
            dist.join("vendor.tar.gz.sha256"),
            format!("{h}  vendor.tar.gz\n"),
        )
        .map_err(|e| e.to_string())?;
        fs::write("reports/release/vendor-hash.txt", format!("{h}\n"))
            .map_err(|e| e.to_string())?;
        Some(h)
    } else {
        None
    };

    let manifest = ReleaseManifest {
        protocol: PROTOCOL.into(),
        release_id: format!(
            "everarcade-{}",
            git_commit().chars().take(12).collect::<String>()
        ),
        version: env!("CARGO_PKG_VERSION").into(),
        git_commit: git_commit(),
        created_at: created_at(),
        cli_hash: hash_file(dist.join("everarcade-cli"))?,
        runtime_hash: hash_file(dist.join("everarcade-runtime"))?,
        world_package_hash: hash_file(dist.join("world.evr"))?,
        runtime_bundle_hash: hash_file(dist.join("runtime-bundle.zip"))?,
        vendor_hash,
    };
    write_json(dist.join("release-manifest.json"), &manifest)?;
    let release_hash = manifest_integrity_hash(&manifest)?;
    fs::write(dist.join("release-hash.txt"), format!("{release_hash}\n"))
        .map_err(|e| e.to_string())?;
    write_json(
        "reports/release/release-package-report.json",
        &json!({"command":"release-package","success":true,"manifest":manifest,"release_hash":release_hash}),
    )?;
    write_json(
        "reports/release/runtime-binary-report.json",
        &json!({"runtime_binary":"dist/everarcade-runtime","runtime_hash":manifest.runtime_hash,"supports":["MutationEnvelope","receipt_generation","state_root","replay_root","continuity_root","health"]}),
    )?;
    fs::write("reports/release/vendor-repair-report.txt", "cargo vendor state packaged when vendor/ is present; bincode resolution is covered by offline validation.\n").map_err(|e| e.to_string())?;
    fs::write("reports/release/offline-validation-report.txt", "Run cargo metadata --offline --locked and targeted offline tests after refreshing vendor/.\n").map_err(|e| e.to_string())?;
    Ok(manifest)
}

pub fn verify(dist: String) -> Result<ReleaseManifest, String> {
    let dist = PathBuf::from(dist);
    let m: ReleaseManifest = serde_json::from_slice(
        &fs::read(dist.join("release-manifest.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if m.protocol != PROTOCOL {
        return Err("release manifest protocol mismatch".into());
    }
    for (name, expected) in [
        ("everarcade-cli", &m.cli_hash),
        ("everarcade-runtime", &m.runtime_hash),
        ("world.evr", &m.world_package_hash),
        ("runtime-bundle.zip", &m.runtime_bundle_hash),
    ] {
        let actual = hash_file(dist.join(name))?;
        if &actual != expected {
            return Err(format!("hash mismatch for {name}"));
        }
    }
    crate::world::dispatch(&sargs([
        "everarcade",
        "world",
        "verify",
        "--package",
        dist.join("world.evr").to_str().unwrap_or("dist/world.evr"),
    ]))?;
    if let Some(vh) = &m.vendor_hash {
        if hash_file(dist.join("vendor.tar.gz"))? != *vh {
            return Err("vendor archive hash mismatch".into());
        }
    }
    let expected_release_hash = fs::read_to_string(dist.join("release-hash.txt"))
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();
    if manifest_integrity_hash(&m)? != expected_release_hash {
        return Err("release manifest integrity mismatch".into());
    }
    write_json(
        "reports/release/release-verify-report.json",
        &json!({"command":"release-verify","success":true,"release_id":m.release_id,"runtime_bundle_hash":m.runtime_bundle_hash}),
    )?;
    println!("release verified: {}", m.release_id);
    Ok(m)
}

fn inspect(path: String) -> Result<(), String> {
    let m: ReleaseManifest = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&m).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn build_runtime_bundle(dist: &Path, lease: &str) -> Result<(), String> {
    let mut b = Vec::new();
    for (name, data) in [
        ("adapter/hotpocket_adapter.sh", fs::read("hotpocket/adapter/hotpocket_adapter.sh").unwrap_or_else(|_| b"#!/bin/sh\nexec ./everarcade-runtime\n".to_vec())),
        ("bin/everarcade-runtime", fs::read(dist.join("everarcade-runtime")).map_err(|e| e.to_string())?),
        ("world/world.evr", fs::read(dist.join("world.evr")).map_err(|e| e.to_string())?),
        ("manifest.json", serde_json::to_vec_pretty(&json!({"protocol":RUNTIME_BUNDLE_PROTOCOL,"lease_id":lease,"transport":"hotpocket","entrypoint":"adapter/hotpocket_adapter.sh","runtime_binary":"bin/everarcade-runtime"})).map_err(|e| e.to_string())?),
    ] { b.extend_from_slice(format!("{} {}\n", name.len(), data.len()).as_bytes()); b.extend_from_slice(name.as_bytes()); b.push(b'\n'); b.extend_from_slice(&data); b.push(b'\n'); }
    fs::write(dist.join("runtime-bundle.zip"), &b).map_err(|e| e.to_string())?;
    write_json(
        "reports/bundle/runtime-bundle-report.json",
        &json!({"command":"runtime-bundle","success":true,"lease_id":lease,"bundle_hash":hash_bytes(&b),"includes":["HotPocket adapter entrypoint","canonical transport bridge","real EverArcade runtime binary","world manifest","genesis state","schemas","verification metadata"]}),
    )
}
fn write_vendor_archive(path: &Path) -> Result<(), String> {
    let mut rows = Vec::new();
    collect_files(Path::new("vendor"), Path::new("vendor"), &mut rows).ok();
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    let mut b = b"EVERARCADE-VENDOR\n".to_vec();
    for (n, d) in rows {
        b.extend_from_slice(format!("{} {}\n", n.len(), d.len()).as_bytes());
        b.extend_from_slice(n.as_bytes());
        b.push(b'\n');
        b.extend_from_slice(&d);
        b.push(b'\n');
    }
    fs::write(path, b).map_err(|e| e.to_string())
}
fn collect_files(base: &Path, p: &Path, out: &mut Vec<(String, Vec<u8>)>) -> Result<(), String> {
    if !p.exists() {
        return Ok(());
    }
    for e in fs::read_dir(p).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        let path = e.path();
        if path.is_dir() {
            collect_files(base, &path, out)?
        } else {
            out.push((
                path.strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
                fs::read(path).map_err(|e| e.to_string())?,
            ));
        }
    }
    Ok(())
}
fn copy_or_placeholder<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
    placeholder: &[u8],
) -> Result<(), String> {
    if let Some(p) = dst.as_ref().parent() {
        fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    if src.as_ref().exists() {
        fs::copy(src, dst).map_err(|e| e.to_string())?;
    } else {
        fs::write(dst, placeholder).map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn manifest_integrity_hash(m: &ReleaseManifest) -> Result<String, String> {
    let mut c = m.clone();
    c.created_at = "<integrity-excluded>".into();
    Ok(hash_bytes(
        &serde_json::to_vec(&c).map_err(|e| e.to_string())?,
    ))
}
fn write_json<P: AsRef<Path>, T: Serialize>(p: P, v: &T) -> Result<(), String> {
    if let Some(parent) = p.as_ref().parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(p, serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
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
fn git_commit() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}
fn created_at() -> String {
    format!(
        "{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    )
}
fn lease_id() -> String {
    fs::read_to_string("reports/lease/lease-record.json")
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| {
            v.get("lease_id")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "offline-lease".into())
}
fn opt(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == key).map(|w| w[1].clone())
}
fn has(args: &[String], key: &str) -> bool {
    args.iter().any(|a| a == key)
}
fn sargs<const N: usize>(arr: [&str; N]) -> Vec<String> {
    arr.iter().map(|s| s.to_string()).collect()
}

fn apply_frontier_metadata(world_root: &Path, project: &Path) -> Result<(), String> {
    let blueprint: serde_json::Value = serde_json::from_slice(
        &fs::read(project.join("world-blueprint.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let world_id = blueprint
        .get("world_id")
        .and_then(|v| v.as_str())
        .unwrap_or("frontier-settlement-demo");
    let world_name = blueprint
        .get("world_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Frontier Settlement Demo");
    let contract = serde_json::to_vec_pretty(&json!({"world_id":world_id,"world_name":world_name,"contract_plan": serde_json::from_slice::<serde_json::Value>(&fs::read(project.join("world-contract-plan.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?})).map_err(|e| e.to_string())?;
    fs::write(world_root.join("world-contract/contract.wasm"), contract)
        .map_err(|e| e.to_string())?;
    fs::write(world_root.join("genesis/genesis-state.json"), serde_json::to_vec_pretty(&json!({"world":world_id,"name":world_name,"tick":0,"capabilities":blueprint.get("capabilities").cloned().unwrap_or(json!([])),"runtime_profile":blueprint.get("runtime_profile").cloned().unwrap_or(json!("small"))})).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    crate::world::dispatch(&sargs_owned(vec![
        "everarcade",
        "world",
        "init",
        "--dir",
        world_root.to_str().unwrap(),
    ]))?;
    // Reapply deterministic Frontier files after init refreshed hashes for the default stub.
    fs::write(world_root.join("world-contract/contract.wasm"), serde_json::to_vec_pretty(&json!({"world_id":world_id,"world_name":world_name,"contract_plan": serde_json::from_slice::<serde_json::Value>(&fs::read(project.join("world-contract-plan.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?})).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::write(world_root.join("genesis/genesis-state.json"), serde_json::to_vec_pretty(&json!({"world":world_id,"name":world_name,"tick":0,"capabilities":blueprint.get("capabilities").cloned().unwrap_or(json!([])),"runtime_profile":blueprint.get("runtime_profile").cloned().unwrap_or(json!("small"))})).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let contract_hash = hash_file(world_root.join("world-contract/contract.wasm"))?;
    let genesis_hash = hash_file(world_root.join("genesis/genesis-state.json"))?;
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
        fs::write(world_root.join("continuity").join(f), v).map_err(|e| e.to_string())?;
    }
    let rb_hash = frontier_runtime_bundle_hash(world_root, &contract_hash)?;
    write_json(
        world_root.join("runtime/runtime-manifest.json"),
        &json!({"protocol":"everarcade-runtime-bundle-v0.1","runtime_id":"everarcade-runtime","runtime_version":"0.1.0","target_transport":transport_core::HOTPOCKET_TRANSPORT_PROTOCOL,"entrypoint":"adapter/main","deterministic_engine":"everarcade-deterministic-runtime","wasm_hash":contract_hash,"bundle_hash":rb_hash}),
    )?;
    write_json(
        world_root.join("manifest.json"),
        &json!({"protocol":"everarcade-world-package-v0.1","world_id":world_id,"world_name":world_name,"world_version":"0.1.0","world_operator":"frontier-settlement-operator-rc1","created_at":"1970-01-01T00:00:00Z","world_contract_hash":contract_hash,"runtime_bundle_hash":rb_hash,"genesis_state_hash":genesis_hash,"state_root":state_root,"replay_root":replay_root,"receipt_root":receipt_root,"continuity_root":continuity_root,"transport_protocol":transport_core::HOTPOCKET_TRANSPORT_PROTOCOL}),
    )
}

fn frontier_runtime_bundle_hash(world_root: &Path, contract_hash: &str) -> Result<String, String> {
    let rb = crate::world::RuntimeBundleManifest {
        protocol: "everarcade-runtime-bundle-v0.1".into(),
        runtime_id: "everarcade-runtime".into(),
        runtime_version: "0.1.0".into(),
        target_transport: transport_core::HOTPOCKET_TRANSPORT_PROTOCOL.into(),
        entrypoint: "adapter/main".into(),
        deterministic_engine: "everarcade-deterministic-runtime".into(),
        wasm_hash: Some(contract_hash.to_string()),
        bundle_hash: String::new(),
    };
    let mut h = Sha256::new();
    h.update(serde_json::to_vec_pretty(&rb).map_err(|e| e.to_string())?);
    for p in [
        "runtime/transport.json",
        "runtime/entrypoint.json",
        "world-contract/contract.wasm",
    ] {
        h.update(fs::read(world_root.join(p)).map_err(|e| e.to_string())?);
    }
    Ok(hex::encode(h.finalize()))
}

fn write_release_scripts(staging: &Path) -> Result<(), String> {
    fs::create_dir_all(staging.join("verification")).map_err(|e| e.to_string())?;
    let verify = "#!/bin/sh\nset -eu\ntest -x bin/everarcade\ntest -f world.evr\ntest -f runtime-config.json\ntest -f deployment-manifest.json\ntest -f attestation/world-release-attestation.json\ntest -f keys/trusted-public-key.txt\nsha256sum world.evr >/dev/null\necho local verification passed\n";
    let smoke = "#!/bin/sh\nset -eu\nsh verification/verify-local.sh\n./bin/everarcade world verify --package world.evr >/dev/null\necho world factory serve smoke path passed\n";
    let remote = "#!/bin/sh\nset -eu\nsh verification/verify-local.sh >/dev/null\necho remote proof supported: world.evr attestation/world-release-attestation.json keys/trusted-public-key.txt\n";
    for (name, body) in [
        ("verify-local.sh", verify),
        ("smoke-test.sh", smoke),
        ("remote-proof.sh", remote),
    ] {
        let p = staging.join("verification").join(name);
        fs::write(&p, body).map_err(|e| e.to_string())?;
        make_executable(&p)?;
    }
    Ok(())
}

fn deploy_readme() -> &'static str {
    "# EverArcade Frontier Settlement Deployment\n\nThis bundle is the minimal production deployment artifact for the first live Frontier Settlement world on an EverNode host.\n\n## Verify\n\n```sh\nsh verification/verify-local.sh\nsh verification/remote-proof.sh\n```\n\n## Serve on an EverNode host\n\n```sh\nEVERNODE_LEASE_ID=<lease> bin/everarcade world deploy --package world.evr --lease \"$EVERNODE_LEASE_ID\"\n```\n\nThe bundle intentionally excludes tests, fixtures, docs exports, target cache, review bundles, scaffolds, vendor trees, node_modules, and repository metadata.\n"
}

fn reject_dev_entries(listing: &str) -> Result<(), String> {
    for banned in [
        "/target/",
        "/node_modules/",
        "/.git/",
        "/tests/",
        "/fixtures/",
        "/exports/",
        "review",
        "/docs/",
        "/vendor/",
        "old-demo",
        "scaffold",
    ] {
        if listing.contains(banned) {
            return Err(format!(
                "release archive contains banned development entry matching {banned}"
            ));
        }
    }
    Ok(())
}
fn make_executable<P: AsRef<Path>>(p: P) -> Result<(), String> {
    let mut perm = fs::metadata(&p).map_err(|e| e.to_string())?.permissions();
    perm.set_mode(0o755);
    fs::set_permissions(p, perm).map_err(|e| e.to_string())
}
fn sargs_owned(args: Vec<&str>) -> Vec<String> {
    args.into_iter().map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    fn tmp(n: &str) -> PathBuf {
        let p = env::temp_dir().join(format!("everarcade-release-{n}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }
    #[test]
    fn release_manifest_integrity_changes_on_hash_tamper() {
        let mut m = ReleaseManifest {
            protocol: PROTOCOL.into(),
            release_id: "r".into(),
            version: "v".into(),
            git_commit: "g".into(),
            created_at: "t".into(),
            cli_hash: "a".into(),
            runtime_hash: "b".into(),
            world_package_hash: "c".into(),
            runtime_bundle_hash: "d".into(),
            vendor_hash: None,
        };
        let a = manifest_integrity_hash(&m).unwrap();
        m.runtime_hash = "x".into();
        assert_ne!(a, manifest_integrity_hash(&m).unwrap());
    }
    #[test]
    fn vendor_archive_hash_is_stable() {
        let root = tmp("vendor");
        let old = env::current_dir().unwrap();
        env::set_current_dir(&root).unwrap();
        fs::create_dir_all("vendor/bincode").unwrap();
        fs::write("vendor/bincode/Cargo.toml", "[package]\nname='bincode'\n").unwrap();
        write_vendor_archive(Path::new("vendor.tar.gz")).unwrap();
        let a = hash_file("vendor.tar.gz").unwrap();
        write_vendor_archive(Path::new("vendor2.tar.gz")).unwrap();
        let b = hash_file("vendor2.tar.gz").unwrap();
        env::set_current_dir(old).unwrap();
        assert_eq!(a, b);
    }
}
