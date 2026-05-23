use crate::coordination::capability::CapabilityNegotiationResult;
use crate::hashing::hash_bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Route {
    pub workload_id: String,
    pub protocol_version: String,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoutingManifest {
    pub routes: Vec<Route>,
}

impl RoutingManifest {
    pub fn canonicalize(&mut self) {
        self.routes.sort();
    }

    pub fn canonical_hash(&self) -> String {
        hash_bytes(&bincode::serialize(self).expect("routing serialize failed"))
    }
}

pub fn route_execution(
    workloads: &[(String, String)],
    candidate_nodes: &[String],
    negotiated: &CapabilityNegotiationResult,
) -> RoutingManifest {
    let selected_version = negotiated
        .shared_protocol_versions
        .first()
        .cloned()
        .unwrap_or_default();
    let mut routes = vec![];
    for (idx, (workload_id, required_protocol)) in workloads.iter().enumerate() {
        if required_protocol == &selected_version && !candidate_nodes.is_empty() {
            routes.push(Route {
                workload_id: workload_id.clone(),
                protocol_version: selected_version.clone(),
                node_id: candidate_nodes[idx % candidate_nodes.len()].clone(),
            });
        }
    }
    let mut manifest = RoutingManifest { routes };
    manifest.canonicalize();
    manifest
}
