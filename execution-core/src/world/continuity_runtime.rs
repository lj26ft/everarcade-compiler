use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityVerificationFailure { pub reason: String, pub rejection_root: String }

fn verify_equal<T: Serialize + PartialEq>(a: &T, b: &T, reason: &str) -> Result<String, ContinuityVerificationFailure> {
    if a == b { Ok(hash_bytes(&canonical_encode(&(reason,a)).map_err(|_| ContinuityVerificationFailure{reason:reason.into(),rejection_root:hash_bytes(reason.as_bytes())})?)) }
    else { Err(ContinuityVerificationFailure{reason:reason.into(), rejection_root:hash_bytes(reason.as_bytes())}) }
}

pub fn verify_world_continuity<T: Serialize + PartialEq>(a: &T, b: &T) -> Result<String, ContinuityVerificationFailure> { verify_equal(a,b,"world_continuity") }
pub fn verify_restore_equivalence<T: Serialize + PartialEq>(a: &T, b: &T) -> Result<String, ContinuityVerificationFailure> { verify_equal(a,b,"restore_equivalence") }
pub fn verify_replay_equivalence<T: Serialize + PartialEq>(a: &T, b: &T) -> Result<String, ContinuityVerificationFailure> { verify_equal(a,b,"replay_equivalence") }
pub fn verify_inventory_equivalence<T: Serialize + PartialEq>(a: &T, b: &T) -> Result<String, ContinuityVerificationFailure> { verify_equal(a,b,"inventory_equivalence") }
pub fn verify_entity_equivalence<T: Serialize + PartialEq>(a: &T, b: &T) -> Result<String, ContinuityVerificationFailure> { verify_equal(a,b,"entity_equivalence") }
pub fn verify_validation_root_chain(a: &[String], b: &[String]) -> Result<String, ContinuityVerificationFailure> { verify_equal(&a,&b,"validation_root_chain") }
