use crate::stable_hash;

pub fn deterministic_signature(package_hash: &str, public_key: &str) -> String { stable_hash(&["registry-signature", package_hash, public_key]) }
