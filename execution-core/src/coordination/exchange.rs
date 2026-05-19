use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{federation::node::FederationNodeId, operator::continuity::Hash256};

use super::{errors::CoordinationError, session::CoordinationSession};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationExchange {
    pub proposal_id: Hash256,
    pub exchanged_by: FederationNodeId,
}

pub fn verify_coordination_exchange(
    exchange: &CoordinationExchange,
    session: &CoordinationSession,
    proposals: &BTreeMap<Hash256, Hash256>,
    seen_exchanges: &BTreeMap<Hash256, CoordinationExchange>,
) -> Result<(), CoordinationError> {
    if !proposals.contains_key(&exchange.proposal_id) {
        return Err(CoordinationError::ProposalMissing);
    }
    if seen_exchanges.contains_key(&exchange.proposal_id) {
        return Err(CoordinationError::DuplicateExchange);
    }
    if !session.participants.contains(&exchange.exchanged_by) {
        return Err(CoordinationError::ExchangeActorNotInSession);
    }
    Ok(())
}
