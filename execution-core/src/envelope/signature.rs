use super::{errors::EnvelopeError, message::SignedContinuityMessage};
use crate::{federation::node::FederationNodeId, operator::continuity::Hash256};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySignature {
    pub signer: FederationNodeId,
    pub signature_hash: Hash256,
}
pub fn verify_continuity_signature(
    signature: &ContinuitySignature,
    message: &SignedContinuityMessage,
) -> Result<(), EnvelopeError> {
    if signature.signer != message.sender {
        return Err(EnvelopeError::SignatureSignerMismatch);
    }
    if signature.signature_hash == [0u8; 32] {
        return Err(EnvelopeError::SignatureHashInvalid);
    }
    Ok(())
}
