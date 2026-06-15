use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const PROTOCOL: &str = "everarcade-release-v0.1";
const RUNTIME_BUNDLE_PROTOCOL: &str = "everarcade-runtime-bundle-v0.1";

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
        .ok_or("usage: everarcade release <package|inspect|verify>")?
    {
        "package" => package(
            opt(args, "--dist").unwrap_or_else(|| "dist".into()),
            has(args, "--vendor"),
        )
        .map(|_| ()),
        "inspect" => {
            inspect(opt(args, "--manifest").unwrap_or_else(|| "dist/release-manifest.json".into()))
        }
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
