use serde::{Deserialize, Serialize};

use super::{hash_hex, BootstrapManifest, RuntimeNodeIdentity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapExecutionState {
    pub node_identity: String,
    pub continuity_root: String,
    pub checkpoint_root: String,
    pub operational_ledger_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapValidationResult {
    pub valid: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapExecutionReceipt {
    pub operation_hash: String,
    pub continuity_root: String,
    pub manifest_hash: String,
    pub runtime_image_hash: String,
    pub checkpoint_root: String,
    pub deterministic_status: String,
}

pub struct RuntimeBootstrapEngine;

impl RuntimeBootstrapEngine {
    pub fn execute(
        manifest: &BootstrapManifest,
        runtime_image_hash: &str,
    ) -> (BootstrapExecutionState, BootstrapExecutionReceipt) {
        let manifest_hash = hash_hex(bincode::serialize(manifest).expect("manifest serializable"));
        let identity = RuntimeNodeIdentity::new(
            manifest.genesis_hash.clone(),
            manifest.expected_package_hash.clone(),
        );
        let continuity_root = hash_hex(format!(
            "continuity:{}:{}",
            identity.deterministic_id, manifest_hash
        ));
        let checkpoint_root = hash_hex(format!(
            "checkpoint:{}:{}",
            continuity_root, runtime_image_hash
        ));
        let operational_ledger_root = hash_hex(format!("ledger:{}", checkpoint_root));
        let operation_hash = hash_hex(format!(
            "bootstrap:{}:{}:{}",
            manifest_hash, runtime_image_hash, checkpoint_root
        ));
        let state = BootstrapExecutionState {
            node_identity: identity.deterministic_id,
            continuity_root: continuity_root.clone(),
            checkpoint_root: checkpoint_root.clone(),
            operational_ledger_root,
        };
        let receipt = BootstrapExecutionReceipt {
            operation_hash,
            continuity_root,
            manifest_hash,
            runtime_image_hash: runtime_image_hash.to_string(),
            checkpoint_root,
            deterministic_status: "DETERMINISTIC_OK".to_string(),
        };
        (state, receipt)
    }

    pub fn validate(manifest: &BootstrapManifest) -> BootstrapValidationResult {
        if manifest.genesis_hash.is_empty() || manifest.expected_package_hash.is_empty() {
            return BootstrapValidationResult {
                valid: false,
                reason: Some("manifest fields cannot be empty".to_string()),
            };
        }
        BootstrapValidationResult {
            valid: true,
            reason: None,
        }
    }
}
