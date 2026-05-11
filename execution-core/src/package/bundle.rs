use crate::state_engine::{proof::MerkleProof, snapshot::StateSnapshot};
use crate::verifier::node::ContractWasm;
use crate::{hashing, ExecutionPlan, ExecutionReceipt, ABI_VERSION};

use super::manifest::ExecutionManifest;

#[derive(Debug, Clone)]
pub enum PackageError {
    Serialization(String),
    Deserialization(String),
    InvalidManifest(&'static str),
    InvalidHash(&'static str),
    EpochMismatch { expected: u64, found: u64 },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionPackage {
    pub manifest: ExecutionManifest,
    pub dag: ExecutionPlan,
    pub contracts: Vec<ContractWasm>,
    pub snapshot: StateSnapshot,
    pub receipts: Vec<ExecutionReceipt>,
    pub proofs: Vec<MerkleProof>,
}

impl ExecutionPackage {
    pub fn recompute_package_hash(&self) -> Result<String, PackageError> {
        let mut clone = self.clone();
        clone.manifest.package_hash.clear();
        serde_json::to_vec(&clone)
            .map(|bytes| hashing::hash_bytes(&bytes))
            .map_err(|e| PackageError::Serialization(e.to_string()))
    }

    pub fn verify_manifest_consistency(&self) -> Result<(), PackageError> {
        if self.manifest.abi_version != ABI_VERSION {
            return Err(PackageError::InvalidManifest("abi_version"));
        }
        if self.manifest.state_root != self.snapshot.state_root {
            return Err(PackageError::InvalidManifest("state_root"));
        }
        let computed = self.recompute_package_hash()?;
        if self.manifest.package_hash != computed {
            return Err(PackageError::InvalidHash("package_hash"));
        }
        Ok(())
    }
}
