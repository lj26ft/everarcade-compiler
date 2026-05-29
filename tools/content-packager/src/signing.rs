use crate::stable_hash;

pub fn sign_package(package_hash: &str, creator_key: &str) -> String { stable_hash(&["signature", package_hash, creator_key]) }
