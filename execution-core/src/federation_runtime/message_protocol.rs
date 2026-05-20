use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::error::FederationRuntimeError;

pub const FEDERATION_PROTOCOL_VERSION: u32 = 1;
pub const MAX_BUNDLE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolEnvelope {
    pub version: u32,
    pub message: FederationProtocolMessage,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FederationProtocolMessage {
    PeerHello {
        node_id: [u8; 32],
        topology_epoch: u64,
    },
    PeerStatus {
        continuity_root: [u8; 32],
        checkpoint_root: [u8; 32],
    },
    CheckpointRequest {
        checkpoint_id: [u8; 32],
    },
    CheckpointResponse {
        checkpoint_id: [u8; 32],
        checkpoint_bytes: Vec<u8>,
    },
    JournalRangeRequest {
        start: u64,
        end: u64,
    },
    JournalRangeResponse {
        start: u64,
        end: u64,
        journal_bytes: Vec<u8>,
    },
    ReceiptBundleRequest {
        receipt_root: [u8; 32],
    },
    ReceiptBundleResponse {
        receipt_root: [u8; 32],
        bundle_bytes: Vec<u8>,
    },
    ReplayProofRequest {
        execution_hash: [u8; 32],
    },
    ReplayProofResponse {
        execution_hash: [u8; 32],
        proof_bytes: Vec<u8>,
    },
    DivergenceReport {
        reason: String,
    },
    ReconciliationRequest {
        target_root: [u8; 32],
    },
}

pub fn validate_protocol_message(
    envelope: &ProtocolEnvelope,
) -> Result<(), FederationRuntimeError> {
    if envelope.version != FEDERATION_PROTOCOL_VERSION {
        return Err(FederationRuntimeError::Serialization(
            "invalid protocol version".into(),
        ));
    }
    Ok(())
}

pub fn validate_bundle_size(bytes: &[u8]) -> Result<(), FederationRuntimeError> {
    if bytes.len() > MAX_BUNDLE_BYTES {
        return Err(FederationRuntimeError::Serialization(
            "oversized bundle".into(),
        ));
    }
    Ok(())
}

pub fn canonical_serialize(envelope: &ProtocolEnvelope) -> Result<Vec<u8>, FederationRuntimeError> {
    validate_protocol_message(envelope)?;
    serde_json::to_vec(envelope).map_err(|e| FederationRuntimeError::Serialization(e.to_string()))
}

pub fn deterministic_hash(envelope: &ProtocolEnvelope) -> Result<[u8; 32], FederationRuntimeError> {
    let bytes = canonical_serialize(envelope)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let out = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&out);
    Ok(hash)
}
