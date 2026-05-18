use std::collections::BTreeSet;

use crate::federation::node::FederationNodeId;

use super::errors::{Result, TopologyError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    pub path: Vec<FederationNodeId>,
}

pub fn deterministic_route(path: Vec<FederationNodeId>) -> Result<Route> {
    if path.is_empty() {
        return Err(TopologyError::EmptyRoute);
    }
    let mut seen = BTreeSet::new();
    for node in &path {
        if !seen.insert(*node) {
            return Err(TopologyError::RouteCycle);
        }
    }
    Ok(Route { path })
}
