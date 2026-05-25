use crate::world::{metrics::RuntimeMetrics, validation::RuntimeValidationRoot};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicApiSurfaceHash(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolSurfaceHash(pub String);

pub fn runtime_validation_root(metrics: &RuntimeMetrics) -> Result<RuntimeValidationRoot, String> {
    crate::world::validation::runtime_validation_root(metrics)
}

pub fn public_api_surface_hash() -> PublicApiSurfaceHash {
    PublicApiSurfaceHash("execution-core-api-v0".to_string())
}

pub fn protocol_surface_hash() -> ProtocolSurfaceHash {
    ProtocolSurfaceHash("execution-core-protocol-v0".to_string())
}
