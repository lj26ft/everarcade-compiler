use std::collections::BTreeMap;

use crate::federation::{node::FederationNodeId, peer::PeerDescriptor};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FederationRegistry {
    peers: BTreeMap<FederationNodeId, PeerDescriptor>,
}

impl FederationRegistry {
    pub fn insert(&mut self, descriptor: PeerDescriptor) -> Option<PeerDescriptor> {
        self.peers.insert(descriptor.identity.node_id, descriptor)
    }

    pub fn get(&self, node_id: &FederationNodeId) -> Option<&PeerDescriptor> {
        self.peers.get(node_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FederationNodeId, &PeerDescriptor)> {
        self.peers.iter()
    }
}
