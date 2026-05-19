use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityEnvelopeDescriptor {
    pub envelope_id: Hash256,
    pub message_hash: Hash256,
    pub registry_hash: Hash256,
}
pub fn hash_envelope_descriptor(descriptor: &ContinuityEnvelopeDescriptor) -> Hash256 {
    Sha256::digest(&canonical_encode(descriptor).expect("envelope descriptor encode")).into()
}
