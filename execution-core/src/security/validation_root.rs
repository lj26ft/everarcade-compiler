use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityValidationRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GovernanceValidationRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IsolationValidationRoot(pub String);
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SecurityValidationRoot(pub String);

impl SecurityValidationRoot {
    pub fn derive(
        capability: &CapabilityValidationRoot,
        governance: &GovernanceValidationRoot,
        isolation: &IsolationValidationRoot,
    ) -> Self {
        SecurityValidationRoot(hash_bytes(
            format!("{}:{}:{}", capability.0, governance.0, isolation.0).as_bytes(),
        ))
    }
}
