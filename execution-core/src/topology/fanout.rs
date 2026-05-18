use crate::federation::node::FederationNodeId;

use super::{mesh::ObserverMesh, neighbors::deterministic_neighbors};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FanoutPlan {
    pub targets: Vec<FederationNodeId>,
}

pub fn deterministic_fanout(
    mesh: &ObserverMesh,
    source: FederationNodeId,
    fanout_limit: usize,
) -> FanoutPlan {
    let mut targets: Vec<_> = deterministic_neighbors(mesh, source, fanout_limit)
        .neighbors
        .into_iter()
        .collect();
    targets.sort();
    FanoutPlan { targets }
}
