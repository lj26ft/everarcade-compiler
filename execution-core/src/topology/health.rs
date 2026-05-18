use crate::federation::node::FederationNodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObserverHealth {
    pub node_id: FederationNodeId,
    pub synchronized: bool,
    pub latest_sequence: u64,
}

pub fn verify_observer_health(health: &ObserverHealth) -> bool {
    health.synchronized || health.latest_sequence > 0
}
