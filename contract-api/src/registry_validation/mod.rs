use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryValidationInput {
    pub abi_version: String,
    pub expected_abi_version: String,
    pub record_types: Vec<String>,
    pub expected_hash: String,
    pub actual_hash: String,
    pub dependencies: Vec<String>,
    pub protocol_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryValidationReport {
    pub abi_compatible: bool,
    pub record_compatible: bool,
    pub hash_valid: bool,
    pub dependencies_valid: bool,
    pub protocol_version_valid: bool,
}

impl RegistryValidationReport {
    pub fn accepted(&self) -> bool {
        self.abi_compatible
            && self.record_compatible
            && self.hash_valid
            && self.dependencies_valid
            && self.protocol_version_valid
    }
}

pub fn validate_registry_package(input: &RegistryValidationInput) -> RegistryValidationReport {
    RegistryValidationReport {
        abi_compatible: input.abi_version == input.expected_abi_version,
        record_compatible: !input.record_types.is_empty()
            && input.record_types.iter().all(|record| record.ends_with("Record")),
        hash_valid: !input.expected_hash.is_empty() && input.expected_hash == input.actual_hash,
        dependencies_valid: input.dependencies.iter().all(|dep| !dep.trim().is_empty()),
        protocol_version_valid: input.protocol_version == "everarcade-protocol-1",
    }
}
