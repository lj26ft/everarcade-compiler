use std::collections::BTreeSet;

use crate::{federation::node::FederationNodeId, sync::advertisement::ContinuityAdvertisement};

use super::{
    errors::{Result, TopologyError},
    mesh::ObserverMesh,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GossipAdvertisement {
    pub source: FederationNodeId,
    pub advertisement: ContinuityAdvertisement,
}

pub fn verify_gossip(
    mesh: &ObserverMesh,
    gossip: &GossipAdvertisement,
    seen: &BTreeSet<(FederationNodeId, u64)>,
) -> Result<()> {
    if !mesh.observers.contains_key(&gossip.source) {
        return Err(TopologyError::UnknownSource);
    }
    if seen.contains(&(gossip.source, gossip.advertisement.cursor.latest_sequence)) {
        return Err(TopologyError::DuplicatePropagation);
    }
    crate::sync::advertisement::verify_advertisement(&gossip.advertisement)
        .map_err(|_| TopologyError::InvalidAdvertisement)
}
