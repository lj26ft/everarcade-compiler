use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnchorPublicationPayload {
    pub record_type: String,
    pub record_hash: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReceiptAnchorRecord {
    pub receipt_root: String,
    pub replay_root: String,
    pub continuity_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayAnchorRecord {
    pub replay_root: String,
    pub checkpoint_root: String,
    pub continuity_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldAnchorRecord {
    pub world_root: String,
    pub replay_root: String,
    pub continuity_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentAnchorRecord {
    pub deployment_root: String,
    pub package_hash: String,
    pub continuity_root: String,
}

pub trait AnchorPublisher<T: Serialize> {
    fn record_type(&self) -> &'static str;

    fn publication_payload(&self, record: &T) -> AnchorPublicationPayload {
        let payload = serde_json::to_vec(record).expect("anchor record serialization");
        AnchorPublicationPayload {
            record_type: self.record_type().to_owned(),
            record_hash: sha256_hex(&payload),
            payload,
        }
    }

    fn verify_payload(&self, payload: &AnchorPublicationPayload) -> bool {
        payload.record_type == self.record_type()
            && payload.record_hash == sha256_hex(&payload.payload)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReceiptPublisher;
#[derive(Debug, Clone, Copy, Default)]
pub struct ReplayPublisher;
#[derive(Debug, Clone, Copy, Default)]
pub struct CheckpointPublisher;
#[derive(Debug, Clone, Copy, Default)]
pub struct DeploymentPublisher;

impl AnchorPublisher<ReceiptAnchorRecord> for ReceiptPublisher {
    fn record_type(&self) -> &'static str {
        "ReceiptAnchorRecord"
    }
}

impl AnchorPublisher<ReplayAnchorRecord> for ReplayPublisher {
    fn record_type(&self) -> &'static str {
        "ReplayAnchorRecord"
    }
}

impl AnchorPublisher<WorldAnchorRecord> for CheckpointPublisher {
    fn record_type(&self) -> &'static str {
        "WorldAnchorRecord"
    }
}

impl AnchorPublisher<DeploymentAnchorRecord> for DeploymentPublisher {
    fn record_type(&self) -> &'static str {
        "DeploymentAnchorRecord"
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
