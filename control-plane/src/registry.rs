use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GamePackage {
    pub game_id: String,
    pub version: String,
    pub package_hash: String,
    pub deprecated: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeVersion {
    pub version: String,
    pub runtime_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustrigPackageSet {
    pub set_id: String,
    pub package_hashes: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentManifest {
    pub deployment_id: String,
    pub game_id: String,
    pub runtime_version: String,
    pub rustrig_set_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayManifest {
    pub replay_root: String,
    pub byte_len: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointManifest {
    pub checkpoint_root: String,
    pub tick: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistryRecord {
    GamePackage(GamePackage),
    RuntimeVersion(RuntimeVersion),
    RustrigPackageSet(RustrigPackageSet),
    DeploymentManifest(DeploymentManifest),
    ReplayManifest(ReplayManifest),
    CheckpointManifest(CheckpointManifest),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeploymentRegistry {
    pub records: BTreeMap<String, RegistryRecord>,
    pub audit_events: Vec<String>,
}
impl DeploymentRegistry {
    pub fn publish(&mut self, key: impl Into<String>, record: RegistryRecord) {
        let key = key.into();
        self.records.insert(key.clone(), record);
        self.audit_events.push(format!("publish:{key}"));
    }
    pub fn verify(&self, key: &str) -> bool {
        self.records.contains_key(key)
    }
    pub fn rollback(&mut self, key: &str, replacement: RegistryRecord) -> bool {
        self.records.insert(key.to_string(), replacement);
        self.audit_events.push(format!("rollback:{key}"));
        true
    }
    pub fn upgrade(&mut self, key: &str, replacement: RegistryRecord) -> bool {
        self.records.insert(key.to_string(), replacement);
        self.audit_events.push(format!("upgrade:{key}"));
        true
    }
    pub fn deprecate(&mut self, key: &str) -> bool {
        if let Some(RegistryRecord::GamePackage(pkg)) = self.records.get_mut(key) {
            pkg.deprecated = true;
            self.audit_events.push(format!("deprecate:{key}"));
            true
        } else {
            false
        }
    }
    pub fn audit(&self) -> &[String] {
        &self.audit_events
    }
}
