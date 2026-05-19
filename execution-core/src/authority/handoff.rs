use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

use super::{
    epoch::{verify_epoch_transition, AuthorityEpoch},
    errors::AuthorityError,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityHandoff {
    pub from: FederationNodeId,
    pub to: FederationNodeId,
    pub epoch: u64,
    pub checkpoint_root: Hash256,
    pub lineage_hash: Hash256,
}

pub fn hash_authority_handoff(handoff: &AuthorityHandoff) -> Hash256 {
    Sha256::digest(&canonical_encode(handoff).expect("authority handoff encode")).into()
}

pub fn verify_handoff(
    previous: &AuthorityEpoch,
    next: &AuthorityEpoch,
    handoff: &AuthorityHandoff,
    expected_checkpoint_root: Hash256,
    expected_lineage_hash: Hash256,
) -> Result<(), AuthorityError> {
    verify_epoch_transition(previous, next)?;
    if handoff.from != previous.authority {
        return Err(AuthorityError::HandoffSourceMismatch);
    }
    if handoff.to != next.authority {
        return Err(AuthorityError::HandoffDestinationMismatch);
    }
    if handoff.from == handoff.to {
        return Err(AuthorityError::SelfHandoffRejected);
    }
    if handoff.epoch != next.epoch {
        return Err(AuthorityError::HandoffEpochMismatch);
    }
    if handoff.checkpoint_root != expected_checkpoint_root {
        return Err(AuthorityError::CheckpointContinuityMismatch);
    }
    if handoff.lineage_hash != expected_lineage_hash {
        return Err(AuthorityError::LineageContinuityMismatch);
    }
    Ok(())
}
