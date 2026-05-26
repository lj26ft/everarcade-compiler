use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod anchors;
pub mod hooks;
pub mod inventory;
pub mod manifests;
pub mod markets;
pub mod ownership;
pub mod payments;
pub mod settlement;
pub mod vaults;
pub mod world_identity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XrplWitness {
    pub ledger_sequence: u64,
    pub transaction_hash: String,
    pub canonical_payload_hash: String,
}

pub fn canonical_bytes<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).expect("canonical XRPL serialization")
}
