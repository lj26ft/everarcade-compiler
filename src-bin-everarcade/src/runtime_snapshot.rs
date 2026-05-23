use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct ApplianceIdentity {
    appliance_id: String,
    federation_id: String,
    runtime_version: String,
    deterministic_mode: bool,
    topology_role: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct EvernodeRuntimeManifest {
    appliance: ApplianceIdentity,
    resource_expectations: serde_json::Value,
    storage_expectations: serde_json::Value,
    replay_expectations: serde_json::Value,
    federation_participation: serde_json::Value,
}

pub fn runtime_snapshot(config_dir: &str) -> Result<(), String> {
    let root = PathBuf::from(config_dir);
    let runtime: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("runtime.toml")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let federation: toml::Value = toml::from_str(
        &fs::read_to_string(root.join("federation.toml")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let storage: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("storage.toml")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let replay: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("replay.toml")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let evernode: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("evernode.toml")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let topology: toml::Value =
        toml::from_str(&fs::read_to_string(root.join("topology.toml")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let id = ApplianceIdentity {
        appliance_id: evernode
            .get("appliance_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        federation_id: federation
            .get("federation_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        runtime_version: runtime
            .get("runtime_version")
            .and_then(|v| v.as_str())
            .unwrap_or("0")
            .to_string(),
        deterministic_mode: runtime
            .get("deterministic_mode")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        topology_role: federation
            .get("topology_role")
            .and_then(|v| v.as_str())
            .unwrap_or("node")
            .to_string(),
    };
    let manifest = EvernodeRuntimeManifest {
        appliance: id,
        resource_expectations: serde_json::json!({"memory_ceiling_mb": evernode.get("memory_ceiling_mb")}),
        storage_expectations: serde_json::json!(storage),
        replay_expectations: serde_json::json!(replay),
        federation_participation: serde_json::json!({"federation": federation, "topology": topology}),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?
    );
    Ok(())
}
