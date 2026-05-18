use std::collections::BTreeSet;

use crate::federation::node::FederationNodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropagationState {
    pub propagated_to: BTreeSet<FederationNodeId>,
    pub propagation_complete: bool,
}

pub fn advance_propagation(
    state: &mut PropagationState,
    target: FederationNodeId,
    total_observers: usize,
) -> bool {
    let inserted = state.propagated_to.insert(target);
    state.propagation_complete = state.propagated_to.len() >= total_observers;
    inserted
}
