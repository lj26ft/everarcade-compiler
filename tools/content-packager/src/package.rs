use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentPackage { pub package_id: String, pub package_hash: String, pub replay_compatible: bool }

pub fn package_content(package_id: &str, assets: &[&str]) -> ContentPackage { ContentPackage { package_id: package_id.to_owned(), package_hash: stable_hash(assets), replay_compatible: true } }
