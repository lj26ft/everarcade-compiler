use crate::stable_hash;

pub fn manage_package(package_id: &str, content_hash: &str) -> String { stable_hash(&["dashboard-package", package_id, content_hash]) }
