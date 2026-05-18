use std::collections::BTreeSet;

use crate::federation::node::FederationNodeId;

use super::mesh::ObserverMesh;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborSet {
    pub neighbors: BTreeSet<FederationNodeId>,
}

pub fn deterministic_neighbors(
    mesh: &ObserverMesh,
    node_id: FederationNodeId,
    max_neighbors: usize,
) -> NeighborSet {
    let mut all: Vec<FederationNodeId> = mesh.observers.keys().copied().collect();
    all.retain(|id| *id != node_id);
    all.sort();
    NeighborSet {
        neighbors: all.into_iter().take(max_neighbors).collect(),
    }
}
