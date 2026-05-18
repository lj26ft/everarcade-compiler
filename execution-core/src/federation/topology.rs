use std::collections::{BTreeMap, BTreeSet};

use crate::federation::node::FederationNodeId;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FederationTopology {
    adjacency: BTreeMap<FederationNodeId, BTreeSet<FederationNodeId>>,
}

impl FederationTopology {
    pub fn connect(&mut self, from: FederationNodeId, to: FederationNodeId) {
        self.adjacency.entry(from).or_default().insert(to);
    }

    pub fn neighbors_of(&self, node: &FederationNodeId) -> Option<&BTreeSet<FederationNodeId>> {
        self.adjacency.get(node)
    }
}
