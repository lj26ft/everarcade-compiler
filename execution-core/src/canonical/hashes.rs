use sha2::{Digest, Sha256};

use crate::{lineage::ExecutionLineageChain, state::CanonicalState, vm::VmExecutionReceipt};

use super::{encoding::canonical_encode, manifests::CanonicalExecutionManifest};

fn hash_bytes(bytes: &[u8]) -> Hash {
    Sha256::digest(bytes).into()
}

pub type Hash = [u8; 32];

pub fn receipt_hash(receipt: &VmExecutionReceipt) -> Hash {
    hash_bytes(&canonical_encode(receipt).expect("receipt encode"))
}

pub fn lineage_hash(lineage: &ExecutionLineageChain) -> Hash {
    hash_bytes(&canonical_encode(lineage).expect("lineage encode"))
}

pub fn state_root_hash(state: &CanonicalState) -> Hash {
    hash_bytes(&canonical_encode(state).expect("state encode"))
}

pub fn event_hash<T: serde::Serialize>(event: &T) -> Hash {
    hash_bytes(&canonical_encode(event).expect("event encode"))
}

pub fn package_hash(package_bytes: &[u8]) -> Hash {
    hash_bytes(package_bytes)
}

pub fn manifest_hash(manifest: &CanonicalExecutionManifest) -> Hash {
    hash_bytes(&canonical_encode(manifest).expect("manifest encode"))
}
