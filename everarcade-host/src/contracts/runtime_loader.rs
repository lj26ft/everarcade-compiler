use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct RuntimeContract {
    pub id: String,
    pub wasm_path: PathBuf,
    pub manifest_path: PathBuf,
    pub contract_hash: [u8; 32],
}

pub fn discover_contracts(root: &Path) -> Result<Vec<RuntimeContract>, String> {
    let contracts_root = root.join("contracts");
    let mut out = Vec::new();
    if !contracts_root.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(&contracts_root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(id) = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
        else {
            continue;
        };
        let wasm_path = path.join("contract.wasm");
        let manifest_path = path.join("manifest.toml");
        if !wasm_path.exists() || !manifest_path.exists() {
            continue;
        }
        let wasm = fs::read(&wasm_path).map_err(|e| e.to_string())?;
        let contract_hash: [u8; 32] = Sha256::digest(&wasm).into();
        out.push(RuntimeContract {
            id,
            wasm_path,
            manifest_path,
            contract_hash,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}
