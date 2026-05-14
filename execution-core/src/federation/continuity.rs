use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityGuarantee {
    pub continuity_hash: String,
}

impl ContinuityGuarantee {
    pub fn from_roots(previous: &str, next: &str) -> Self {
        Self {
            continuity_hash: hash_bytes(format!("{previous}->{next}").as_bytes()),
        }
    }
}
