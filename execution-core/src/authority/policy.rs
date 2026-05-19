use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::federation::node::FederationNodeId;

use super::errors::AuthorityError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub single_authority_required: bool,
}

pub fn verify_execution_policy(
    policy: &ExecutionPolicy,
    active_authorities: &[FederationNodeId],
) -> Result<(), AuthorityError> {
    if policy.single_authority_required {
        let unique = active_authorities.iter().copied().collect::<BTreeSet<_>>();
        if unique.len() != 1 || active_authorities.is_empty() {
            return Err(AuthorityError::MultiWriterExecutionRejected);
        }
    }
    Ok(())
}
