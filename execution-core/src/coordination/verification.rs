use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::operator::continuity::Hash256;

use super::{
    errors::CoordinationError, exchange::verify_coordination_exchange,
    policy::verify_coordination_policy, quarantine::verify_coordination_quarantine,
    session::verify_coordination_session, state::verify_coordination_state, CoordinationExchange,
    CoordinationPolicy, CoordinationQuarantine, CoordinationRegistry, CoordinationSession,
    CoordinationState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationVerificationReport {
    pub valid: bool,
    pub quarantine_required: bool,
}

pub fn verify_coordination(
    session: &CoordinationSession,
    exchange: &CoordinationExchange,
    proposals: &BTreeMap<Hash256, Hash256>,
    seen_exchanges: &BTreeMap<Hash256, CoordinationExchange>,
    registry: &CoordinationRegistry,
    policy: &CoordinationPolicy,
    quarantine: &CoordinationQuarantine,
    state: &CoordinationState,
) -> Result<CoordinationVerificationReport, CoordinationError> {
    verify_coordination_session(session)?;
    verify_coordination_policy(policy, true)?;
    let exchange_valid =
        verify_coordination_exchange(exchange, session, proposals, seen_exchanges).is_ok();
    verify_coordination_quarantine(quarantine, exchange_valid)?;
    if exchange_valid {
        verify_coordination_state(state, registry)?;
    }
    Ok(CoordinationVerificationReport {
        valid: exchange_valid,
        quarantine_required: !exchange_valid,
    })
}
