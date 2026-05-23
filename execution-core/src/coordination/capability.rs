use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeCapabilities {
    pub node_id: String,
    pub protocol_versions: Vec<String>,
    pub abi_versions: Vec<String>,
    pub wasm_runtime_versions: Vec<String>,
    pub federation_features: Vec<String>,
    pub replay_compatibility: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityNegotiationResult {
    pub shared_protocol_versions: Vec<String>,
    pub shared_abi_versions: Vec<String>,
    pub shared_wasm_runtime_versions: Vec<String>,
    pub shared_features: Vec<String>,
    pub replay_compatible: bool,
}

impl CapabilityNegotiationResult {
    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("capability negotiation serialize failed"))
    }
}

pub fn negotiate_capabilities(nodes: &[NodeCapabilities]) -> CapabilityNegotiationResult {
    let mut protocol: Option<BTreeSet<String>> = None;
    let mut abi: Option<BTreeSet<String>> = None;
    let mut wasm: Option<BTreeSet<String>> = None;
    let mut features: Option<BTreeSet<String>> = None;
    let mut replay_compatible = true;

    for node in nodes {
        replay_compatible &= node.replay_compatibility;
        let p = node.protocol_versions.iter().cloned().collect();
        let a = node.abi_versions.iter().cloned().collect();
        let w = node.wasm_runtime_versions.iter().cloned().collect();
        let f = node.federation_features.iter().cloned().collect();

        protocol = Some(match protocol {
            Some(acc) => acc.intersection(&p).cloned().collect(),
            None => p,
        });
        abi = Some(match abi {
            Some(acc) => acc.intersection(&a).cloned().collect(),
            None => a,
        });
        wasm = Some(match wasm {
            Some(acc) => acc.intersection(&w).cloned().collect(),
            None => w,
        });
        features = Some(match features {
            Some(acc) => acc.intersection(&f).cloned().collect(),
            None => f,
        });
    }

    CapabilityNegotiationResult {
        shared_protocol_versions: protocol.unwrap_or_default().into_iter().collect(),
        shared_abi_versions: abi.unwrap_or_default().into_iter().collect(),
        shared_wasm_runtime_versions: wasm.unwrap_or_default().into_iter().collect(),
        shared_features: features.unwrap_or_default().into_iter().collect(),
        replay_compatible,
    }
}
