use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::federation::node::FederationNodeId;

use super::{ack::FinalityAcknowledgment, errors::FinalityError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityQuorum {
    pub required_observers: usize,
}

pub fn verify_quorum(
    quorum: &FinalityQuorum,
    acknowledgements: &[FinalityAcknowledgment],
) -> Result<(), FinalityError> {
    let mut seen = BTreeSet::new();
    for ack in acknowledgements {
        if !seen.insert(ack.observer) {
            return Err(FinalityError::DuplicateAcknowledgment);
        }
    }
    if seen.len() < quorum.required_observers {
        return Err(FinalityError::QuorumNotReached);
    }
    Ok(())
}
