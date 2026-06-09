use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package_id: String,
    pub package_version: String,
    pub runtime_compatibility: String,
    pub wasm_path: String,
    pub wasm_hash: String,
    pub signature: String,
    pub world_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoadedPackage {
    pub manifest: PackageManifest,
    pub package_hash: String,
    pub wasm: Vec<u8>,
    pub world_metadata: serde_json::Value,
}

#[derive(Clone, Debug)]
pub struct PackageLoader {
    pub required_runtime: String,
}

impl PackageLoader {
    pub fn new(required_runtime: impl Into<String>) -> Self {
        Self {
            required_runtime: required_runtime.into(),
        }
    }

    pub fn load(&self, package_dir: impl AsRef<Path>) -> Result<LoadedPackage> {
        let package_dir = package_dir.as_ref();
        let manifest_path = package_dir.join("manifest.json");
        let manifest: PackageManifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
        self.validate_manifest(&manifest)?;
        let wasm_path = package_dir.join(&manifest.wasm_path);
        let wasm = fs::read(&wasm_path)?;
        let wasm_hash = hex::encode(Sha256::digest(&wasm));
        if wasm_hash != manifest.wasm_hash {
            return Err(anyhow!("package wasm hash mismatch"));
        }
        self.verify_signature(&manifest)?;
        let world_metadata_path: PathBuf = package_dir.join("world.json");
        let world_metadata = if world_metadata_path.exists() {
            serde_json::from_slice(&fs::read(world_metadata_path)?)?
        } else {
            serde_json::json!({"world_id": manifest.world_id})
        };
        let package_hash = self.package_hash(&manifest, &wasm)?;
        Ok(LoadedPackage {
            manifest,
            package_hash,
            wasm,
            world_metadata,
        })
    }

    pub fn validate_manifest(&self, manifest: &PackageManifest) -> Result<()> {
        if manifest.package_id.trim().is_empty() {
            return Err(anyhow!("missing package id"));
        }
        if manifest.package_version.trim().is_empty() {
            return Err(anyhow!("missing package version"));
        }
        if manifest.runtime_compatibility != self.required_runtime {
            return Err(anyhow!("runtime compatibility mismatch"));
        }
        if manifest.wasm_path.trim().is_empty()
            || manifest.wasm_hash.len() != 64
            || !manifest.wasm_hash.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(anyhow!("invalid wasm reference"));
        }
        if manifest.signature.trim().is_empty() {
            return Err(anyhow!("missing package signature"));
        }
        Ok(())
    }

    fn verify_signature(&self, manifest: &PackageManifest) -> Result<()> {
        let expected = format!("sha256:{}", manifest.wasm_hash);
        if manifest.signature != expected {
            return Err(anyhow!("package signature verification failed"));
        }
        Ok(())
    }

    fn package_hash(&self, manifest: &PackageManifest, wasm: &[u8]) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_vec(manifest)?);
        hasher.update(wasm);
        Ok(hex::encode(hasher.finalize()))
    }
}
