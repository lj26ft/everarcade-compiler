use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolWorldManifest { pub world_id: String, pub state_schema_version: String, pub governance_manifest_hash: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolExecutionManifest { pub runtime_version: String, pub deterministic_constraints_hash: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolGamePackage { pub package_id: String, pub world_manifest: ProtocolWorldManifest, pub execution_manifest: ProtocolExecutionManifest }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolReplayBundle { pub bundle_id: String, pub authority_envelope: ProtocolAuthorityEnvelope, pub replay_root: String }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolAuthorityEnvelope { pub world_id: String, pub tick: u64, pub state_root: String }
