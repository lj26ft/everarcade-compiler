use serde::{Deserialize, Serialize};

use crate::{federation::node::FederationNodeId, operator::continuity::Hash256};

use super::{errors::AuthorityError, registry::AuthorityRegistry};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAuthorityProof {
    pub authority: FederationNodeId,
    pub epoch: u64,
    pub execution_id: Hash256,
}

pub fn verify_execution_authority(
    proof: &ExecutionAuthorityProof,
    registry: &AuthorityRegistry,
) -> Result<(), AuthorityError> {
    if proof.authority != registry.current_authority || proof.epoch != registry.current_epoch {
        return Err(AuthorityError::UnauthorizedExecution);
    }
    if proof.execution_id == [0u8; 32] {
        return Err(AuthorityError::UnauthorizedExecution);
    }
    Ok(())
}
