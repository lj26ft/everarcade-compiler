use std::collections::BTreeSet;

use crate::federation::node::FederationNodeId;

use super::errors::{Result, TopologyError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subscription {
    pub observer: FederationNodeId,
    pub world_id: String,
}

pub fn verify_subscription(
    subscription: &Subscription,
    allowed_worlds: &BTreeSet<String>,
) -> Result<()> {
    if subscription.world_id.trim().is_empty() || !allowed_worlds.contains(&subscription.world_id) {
        return Err(TopologyError::InvalidSubscription);
    }
    Ok(())
}
