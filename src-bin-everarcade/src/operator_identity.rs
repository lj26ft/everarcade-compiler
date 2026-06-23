use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

const REGISTRY_FILE: &str = "operator-registry.json";
const OPERATOR_SCHEMA: &str = "OPERATOR_IDENTITY_RC1";
const REGISTRY_SCHEMA: &str = "OPERATOR_REGISTRY_RC1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatorRecord {
    pub schema_version: String,
    pub operator_id: String,
    pub display_name: String,
    pub role: String,
    pub public_key: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub previous_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegistry {
    pub schema_version: String,
    pub registry_hash: String,
    pub operators: Vec<OperatorRecord>,
}

pub fn dispatch(args: &[String]) -> Result<(), String> {
    match args.get(2).map(String::as_str) {
        Some("registry") => match args.get(3).map(String::as_str) {
            Some("init") => init(),
            Some("verify") => verify_registry_file(
                &opt(args, "--registry").unwrap_or_else(|| REGISTRY_FILE.into()),
            ),
            _ => Err("usage: everarcade operator registry <init|verify>".into()),
        },
        Some("register") => register(args),
        Some("verify") => verify_operator(args),
        Some("revoke") => revoke(args),
        Some("rotate-key") => rotate_key(args),
        _ => Err("usage: everarcade operator <registry|register|verify|revoke|rotate-key>".into()),
    }
}

fn init() -> Result<(), String> {
    let mut registry = OperatorRegistry {
        schema_version: REGISTRY_SCHEMA.into(),
        registry_hash: String::new(),
        operators: vec![],
    };
    registry.registry_hash = registry_hash(&registry.operators)?;
    write_registry(REGISTRY_FILE, &registry)?;
    println!("PASS operator registry initialized path={REGISTRY_FILE}");
    Ok(())
}

fn register(args: &[String]) -> Result<(), String> {
    let mut registry = read_or_empty()?;
    let id = opt(args, "--operator-id").unwrap_or_else(|| "operator.frontier.settlement".into());
    if registry.operators.iter().any(|o| o.operator_id == id) {
        return Err("FAIL duplicate operator_id".into());
    }
    let now = opt(args, "--created-at").unwrap_or_else(|| "2026-06-23T00:00:00Z".into());
    let record = OperatorRecord {
        schema_version: OPERATOR_SCHEMA.into(),
        operator_id: id,
        display_name: opt(args, "--display-name")
            .unwrap_or_else(|| "Frontier Settlement Operator".into()),
        role: opt(args, "--role").unwrap_or_else(|| "world_operator".into()),
        public_key: opt(args, "--public-key")
            .unwrap_or_else(|| "ed25519:frontier-settlement-operator-rc1".into()),
        status: "ACTIVE".into(),
        created_at: now.clone(),
        updated_at: now,
        previous_keys: vec![],
    };
    validate_record(&record)?;
    registry.operators.push(record);
    write_rehashed(&mut registry)?;
    println!("PASS operator registered");
    Ok(())
}

fn verify_operator(args: &[String]) -> Result<(), String> {
    let registry = read_registry(&opt(args, "--registry").unwrap_or_else(|| REGISTRY_FILE.into()))?;
    verify_registry(&registry)?;
    let id = opt(args, "--operator-id").unwrap_or_else(|| "operator.frontier.settlement".into());
    let key = opt(args, "--public-key")
        .unwrap_or_else(|| "ed25519:frontier-settlement-operator-rc1".into());
    let op = registry
        .operators
        .iter()
        .find(|o| o.operator_id == id)
        .ok_or("FAIL operator missing")?;
    if op.status != "ACTIVE" {
        return Err("FAIL operator not active".into());
    }
    if op.public_key != key {
        return Err("FAIL key mismatch".into());
    }
    println!("PASS operator verified");
    Ok(())
}

fn revoke(args: &[String]) -> Result<(), String> {
    let mut registry = read_registry(REGISTRY_FILE)?;
    let id = opt(args, "--operator-id").unwrap_or_else(|| "operator.frontier.settlement".into());
    let op = registry
        .operators
        .iter_mut()
        .find(|o| o.operator_id == id)
        .ok_or("FAIL operator missing")?;
    op.status = "REVOKED".into();
    op.updated_at = opt(args, "--updated-at").unwrap_or_else(|| "2026-06-23T00:00:00Z".into());
    write_rehashed(&mut registry)?;
    println!("PASS operator revoked");
    Ok(())
}

fn rotate_key(args: &[String]) -> Result<(), String> {
    let mut registry = read_registry(REGISTRY_FILE)?;
    let id = opt(args, "--operator-id").unwrap_or_else(|| "operator.frontier.settlement".into());
    let new_key = opt(args, "--new-public-key")
        .unwrap_or_else(|| "ed25519:frontier-settlement-operator-rc1-rotated".into());
    let op = registry
        .operators
        .iter_mut()
        .find(|o| o.operator_id == id)
        .ok_or("FAIL operator missing")?;
    if op.status != "ACTIVE" {
        return Err("FAIL cannot rotate revoked operator".into());
    }
    if op.public_key == new_key || op.previous_keys.contains(&new_key) {
        return Err("FAIL broken rotation chain".into());
    }
    op.previous_keys.push(op.public_key.clone());
    op.public_key = new_key;
    op.updated_at = opt(args, "--updated-at").unwrap_or_else(|| "2026-06-23T00:00:00Z".into());
    write_rehashed(&mut registry)?;
    println!("PASS operator key rotated");
    Ok(())
}

pub fn verify_registry_file(path: &str) -> Result<(), String> {
    verify_registry(&read_registry(path)?)?;
    println!("PASS operator registry verified");
    Ok(())
}

pub fn verify_registry(registry: &OperatorRegistry) -> Result<(), String> {
    if registry.schema_version != REGISTRY_SCHEMA {
        return Err("FAIL invalid registry schema".into());
    }
    if registry.registry_hash != registry_hash(&registry.operators)? {
        return Err("FAIL registry hash mismatch".into());
    }
    let mut ids = BTreeSet::new();
    let mut active_keys = BTreeSet::new();
    for op in &registry.operators {
        validate_record(op)?;
        if !ids.insert(&op.operator_id) {
            return Err("FAIL duplicate operator_id".into());
        }
        if op.status == "ACTIVE" && !active_keys.insert(&op.public_key) {
            return Err("FAIL duplicate active key".into());
        }
        if op.previous_keys.contains(&op.public_key) {
            return Err("FAIL broken rotation chain".into());
        }
    }
    Ok(())
}

fn validate_record(op: &OperatorRecord) -> Result<(), String> {
    if op.schema_version != OPERATOR_SCHEMA {
        return Err("FAIL invalid operator schema".into());
    }
    if !matches!(
        op.role.as_str(),
        "world_operator" | "attester" | "verifier" | "treasury_operator" | "archive_operator"
    ) {
        return Err("FAIL invalid role".into());
    }
    if !matches!(op.status.as_str(), "ACTIVE" | "REVOKED") {
        return Err("FAIL invalid status".into());
    }
    if op.operator_id.is_empty() || op.public_key.is_empty() {
        return Err("FAIL missing operator identity fields".into());
    }
    Ok(())
}

fn registry_hash(operators: &[OperatorRecord]) -> Result<String, String> {
    let mut canonical = operators.to_vec();
    canonical.sort_by(|a, b| {
        a.operator_id
            .cmp(&b.operator_id)
            .then(a.public_key.cmp(&b.public_key))
    });
    let bytes = serde_json::to_vec(&canonical).map_err(|e| e.to_string())?;
    Ok(hex::encode(Sha256::digest(bytes)))
}
fn write_rehashed(registry: &mut OperatorRegistry) -> Result<(), String> {
    registry.registry_hash = registry_hash(&registry.operators)?;
    write_registry(REGISTRY_FILE, registry)
}
fn read_or_empty() -> Result<OperatorRegistry, String> {
    if Path::new(REGISTRY_FILE).exists() {
        read_registry(REGISTRY_FILE)
    } else {
        Ok(OperatorRegistry {
            schema_version: REGISTRY_SCHEMA.into(),
            registry_hash: registry_hash(&[])?,
            operators: vec![],
        })
    }
}
fn read_registry(path: &str) -> Result<OperatorRegistry, String> {
    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn write_registry(path: &str, registry: &OperatorRegistry) -> Result<(), String> {
    fs::write(
        path,
        serde_json::to_string_pretty(registry).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
fn opt(args: &[String], name: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == name).map(|w| w[1].clone())
}
