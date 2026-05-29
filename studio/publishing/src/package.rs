#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishedPackage { pub package_id: String, pub package_hash: String, pub deterministic: bool }
pub fn package_content(package_id: &str, content_hashes: &[&str]) -> PublishedPackage { let mut parts = vec!["publish-package", package_id]; parts.extend_from_slice(content_hashes); PublishedPackage { package_id: package_id.to_owned(), package_hash: crate::stable_hash(&parts), deterministic: true } }
