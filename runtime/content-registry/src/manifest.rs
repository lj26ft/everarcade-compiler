use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegistryManifest { pub package_id: String, pub manifest_hash: String }

pub fn canonical_manifest(package_id: &str, package_hash: &str) -> RegistryManifest { RegistryManifest { package_id: package_id.to_owned(), manifest_hash: stable_hash(&[package_id, package_hash]) } }
