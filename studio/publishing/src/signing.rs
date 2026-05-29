pub fn sign_package(package_hash: &str, creator_key: &str) -> String { crate::stable_hash(&["package-signature", package_hash, creator_key]) }
