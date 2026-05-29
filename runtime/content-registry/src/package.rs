use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisteredPackage { pub package_id: String, pub package_hash: String, pub continuity_root: String }

pub fn register_package(package_id: &str, package_hash: &str) -> RegisteredPackage {
    RegisteredPackage { package_id: package_id.to_owned(), package_hash: package_hash.to_owned(), continuity_root: stable_hash(&["registry", package_id, package_hash]) }
}
