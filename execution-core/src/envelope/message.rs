use super::errors::EnvelopeError;
use crate::{
    canonical::encoding::canonical_encode, federation::node::FederationNodeId,
    operator::continuity::Hash256,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedContinuityMessage {
    pub message_id: Hash256,
    pub sender: FederationNodeId,
    pub payload_hash: Hash256,
}
pub fn hash_signed_message(message: &SignedContinuityMessage) -> Hash256 {
    Sha256::digest(&canonical_encode(message).expect("signed continuity message encode")).into()
}
pub fn verify_signed_message(message: &SignedContinuityMessage) -> Result<(), EnvelopeError> {
    if message.payload_hash == [0u8; 32] {
        return Err(EnvelopeError::PayloadHashInvalid);
    }
    if hash_signed_message(message) != message.message_id {
        return Err(EnvelopeError::MessageIdMismatch);
    }
    Ok(())
}
