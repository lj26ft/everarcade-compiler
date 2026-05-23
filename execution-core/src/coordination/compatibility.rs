use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolCompatibility {
    pub supported_protocol_versions: Vec<String>,
    pub abi_compatibility_hash: String,
    pub execution_compatibility_hash: String,
    pub migration_compatibility_hash: String,
    pub capability_negotiation_root: String,
}

impl ProtocolCompatibility {
    pub fn canonicalize(&mut self) {
        self.supported_protocol_versions.sort();
        self.supported_protocol_versions.dedup();
    }

    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("compatibility serialize failed"))
    }

    pub fn is_protocol_supported(&self, version: &str) -> Result<(), CompatibilityError> {
        if self
            .supported_protocol_versions
            .iter()
            .any(|v| v == version)
        {
            Ok(())
        } else {
            Err(CompatibilityError::UnsupportedProtocol(version.to_string()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
    UnsupportedProtocol(String),
}
