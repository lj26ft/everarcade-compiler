use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};

use super::errors::AuthorityError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityEpoch {
    pub epoch: u64,
    pub authority: FederationNodeId,
    pub previous_epoch_hash: Option<Hash256>,
}

pub fn hash_authority_epoch(epoch: &AuthorityEpoch) -> Hash256 {
    Sha256::digest(&canonical_encode(epoch).expect("authority epoch encode")).into()
}

pub fn verify_epoch_transition(
    previous: &AuthorityEpoch,
    next: &AuthorityEpoch,
) -> Result<(), AuthorityError> {
    if next.epoch <= previous.epoch {
        return Err(AuthorityError::EpochRollback);
    }
    if next.epoch != previous.epoch + 1 {
        return Err(AuthorityError::NonMonotonicEpoch);
    }
    if next.previous_epoch_hash != Some(hash_authority_epoch(previous)) {
        return Err(AuthorityError::PreviousEpochHashMismatch);
    }
    Ok(())
}
